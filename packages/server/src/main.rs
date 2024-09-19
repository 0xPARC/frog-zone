mod zone;

use rocket::http::Method;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time;
use tracing::info;
use zone::{Coord, Direction, Event, Item, Player, Zone};

#[macro_use]
extern crate rocket;

struct GameState {
    zone: Zone,
    move_queue: VecDeque<(usize, Direction)>,
    events: Vec<Event>,
}

type SharedState = Arc<Mutex<GameState>>;

#[derive(Deserialize)]
struct MoveRequest {
    player_id: usize,
    direction: Direction,
}

#[derive(Serialize, Clone)]
struct GameStateResponse {
    events: Vec<Event>,
}

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:5173",
        "http://127.0.0.1:5173",
        "http://localhost:5173",
        "http://0.0.0.0:5173",
    ]);

    CorsOptions {
        // 5.
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Options]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
            "Content-Type",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("[main] error while building CORS")
}

#[post("/move", data = "<move_request>")]
async fn queue_move(state: &State<SharedState>, move_request: Json<MoveRequest>) -> &'static str {
    let mut game_state = state.lock().unwrap();
    game_state
        .move_queue
        .push_back((move_request.player_id, move_request.direction));
    "move queued"
}

#[get("/state")]
async fn get_state(state: &State<SharedState>) -> Json<GameStateResponse> {
    let game_state = state.lock().unwrap();
    Json(GameStateResponse {
        events: game_state.events.clone(),
    })
}

async fn game_loop(state: SharedState) {
    let mut interval = time::interval(Duration::from_millis(state.lock().unwrap().zone.tick_rate));
    loop {
        interval.tick().await;
        let mut game_state = state.lock().unwrap();

        while let Some((player_id, direction)) = game_state.move_queue.pop_front() {
            let new_events = game_state.zone.move_player(player_id, direction);
            game_state.events.extend(new_events);
        }

        info!("processed {} events", game_state.events.len());
    }
}

#[launch]
async fn rocket() -> _ {
    let shared_state: Arc<Mutex<GameState>> = Arc::new(Mutex::new(GameState {
        zone: Zone::new(64, 64, 3_000), // 64x64 zone, 3s tick
        move_queue: VecDeque::new(),
        events: Vec::new(),
    }));

    let mut add_events = Vec::new();
    let mut setup_state_guard = shared_state.lock().unwrap();
    add_events.push(setup_state_guard.zone.add_player(
        Player {
            id: 1,
            atk: 50,
            hp: 100,
        },
        Coord { x: 0, y: 0 },
    ));
    add_events.push(setup_state_guard.zone.add_player(
        Player {
            id: 2,
            atk: 50,
            hp: 100,
        },
        Coord { x: 10, y: 0 },
    ));
    add_events.push(setup_state_guard.zone.add_player(
        Player {
            id: 3,
            atk: 50,
            hp: 100,
        },
        Coord { x: 20, y: 0 },
    ));
    add_events.push(setup_state_guard.zone.add_player(
        Player {
            id: 4,
            atk: 50,
            hp: 100,
        },
        Coord { x: 30, y: 0 },
    ));
    add_events.push(
        setup_state_guard
            .zone
            .add_item(Item { atk: 10, hp: 10 }, Coord { x: 5, y: 5 }),
    );
    add_events.push(
        setup_state_guard
            .zone
            .add_item(Item { atk: 10, hp: 10 }, Coord { x: 10, y: 10 }),
    );
    add_events.push(
        setup_state_guard
            .zone
            .add_item(Item { atk: 10, hp: 10 }, Coord { x: 15, y: 15 }),
    );
    add_events.push(
        setup_state_guard
            .zone
            .add_item(Item { atk: 10, hp: 10 }, Coord { x: 20, y: 20 }),
    );

    setup_state_guard.events.extend(add_events.into_iter());
    drop(setup_state_guard);

    let state_guard = shared_state.clone();
    tokio::spawn(async move {
        game_loop(state_guard).await;
    });

    rocket::build()
        .manage(shared_state.clone())
        .mount("/", routes![queue_move, get_state])
        .attach(make_cors())
}
