mod zone;

use rocket::http::Method;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};
use tracing::info;
use zone::{CellEncryptedData, Direction, Encrypted, EncryptedCoord, Zone};

use std::time::Duration;
use tokio::time;

#[macro_use]
extern crate rocket;

const MOVE_TIME_MILLIS: u64 = 500;
const GET_CELL_TIME_MILLIS: u64 = 140; // based on benchmark of 700ms for 5 cells

struct GameState {
    zone: Zone,
    move_queue: VecDeque<(usize, Encrypted<Direction>, Arc<Notify>)>,
}

type SharedState = Arc<Mutex<GameState>>;

#[derive(Deserialize)]
struct GetCellsRequest {
    player_id: usize,
    coords: Vec<EncryptedCoord>,
}

#[derive(Serialize, Clone)]
struct GetCellsResponse {
    cell_data: Vec<CellEncryptedData>,
}

#[derive(Deserialize)]
struct MoveRequest {
    player_id: usize,
    direction: Encrypted<Direction>,
}

#[derive(Serialize, Clone)]
struct MoveResponse {
    my_new_coords: EncryptedCoord,
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

#[post("/move", format = "json", data = "<move_request>")]
async fn queue_move(
    state: &State<SharedState>,
    move_request: Json<MoveRequest>,
) -> Json<MoveResponse> {
    let notify = Arc::new(Notify::new());
    let notify_clone = notify.clone();

    {
        let mut game_state = state.lock().await;
        game_state.move_queue.push_back((
            move_request.player_id,
            move_request.direction,
            notify_clone,
        ));
    }

    notify.notified().await;

    let game_state = state.lock().await;
    let player = &game_state.zone.players[move_request.player_id];

    info!("processed /move request");

    Json(MoveResponse {
        my_new_coords: player.data.loc,
    })
}

#[post("/get_cells", format = "json", data = "<request>")]
async fn get_cells(
    state: &State<SharedState>,
    request: Json<GetCellsRequest>,
) -> Json<GetCellsResponse> {
    let mut cell_data = Vec::new();

    {
        let game_state = state.lock().await;
        let zone = &game_state.zone;

        cell_data = zone.get_cells(request.player_id, request.coords.clone());
    }

    let len = request.coords.len();
    time::sleep(Duration::from_millis(GET_CELL_TIME_MILLIS * (len as u64))).await;

    info!("processed /get_cells request");

    Json(GetCellsResponse { cell_data })
}

async fn process_moves(state: SharedState) {
    loop {
        let (player_id, direction, notify) = {
            let mut game_state = state.lock().await;
            if let Some(move_request) = game_state.move_queue.pop_front() {
                move_request
            } else {
                // No moves to process, wait for a new move to be queued
                drop(game_state);
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                continue;
            }
        };

        // simulate that the state update does not happen until 500ms has
        // elapsed following a move request
        time::sleep(Duration::from_millis(MOVE_TIME_MILLIS)).await;

        {
            let mut game_state = state.lock().await;
            let zone = &mut game_state.zone;
            zone.move_player(player_id, direction);
        }

        notify.notify_one();
    }
}

#[launch]
async fn rocket() -> _ {
    let shared_state: Arc<Mutex<GameState>> = Arc::new(Mutex::new(GameState {
        zone: Zone::new(64, 64), // 64x64 zone
        move_queue: VecDeque::new(),
    }));

    let state_clone = shared_state.clone();
    tokio::spawn(async move {
        process_moves(state_clone).await;
    });

    rocket::build()
        .manage(shared_state.clone())
        .mount("/", routes![queue_move, get_cells])
        .attach(make_cors())
}
