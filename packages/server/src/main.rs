mod zone;
use phantom::{PhantomEvaluator, PhantomParam, PhantomPk, PhantomRound1Key, PhantomRound2Key};
use rocket::http::Method;
use rocket::response::status::NotFound;
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
const MOVE_TIME_RATE_LIMIT_MILLIS: u64 = 3500;

struct GameState {
    zone: Zone,
    move_queue: VecDeque<(usize, Encrypted<Direction>, Arc<Notify>)>,
    player_last_move_time: [u64; 4],
    // Phantom
    evaluator: PhantomEvaluator,
    player_round_1_key: [Option<PhantomRound1Key>; 4],
    player_round_2_key: [Option<PhantomRound2Key>; 4],
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
    rate_limited: bool,
}

#[derive(Deserialize)]
struct SubmitRound1KeyRequest {
    player_id: usize,
    key: PhantomRound1Key,
}

#[derive(Serialize, Clone)]
struct GetPkResponse {
    pk: PhantomPk,
}

#[derive(Deserialize)]
struct SubmitRound2KeyRequest {
    player_id: usize,
    key: PhantomRound2Key,
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

#[post("/move", format = "json", data = "<move_request>")]
async fn queue_move(
    state: &State<SharedState>,
    move_request: Json<MoveRequest>,
) -> Json<MoveResponse> {
    let can_move = {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let game_state = state.lock().await;
        let last_request_time = game_state.player_last_move_time[move_request.player_id];
        current_time - last_request_time > MOVE_TIME_RATE_LIMIT_MILLIS
    };

    if !can_move {
        return Json(MoveResponse {
            my_new_coords: EncryptedCoord {
                x: Encrypted { val: 0 },
                y: Encrypted { val: 0 },
            },
            rate_limited: true,
        });
    }

    let notify = Arc::new(Notify::new());
    let notify_clone = notify.clone();

    {
        let mut game_state = state.lock().await;
        game_state.move_queue.push_back((
            move_request.player_id,
            move_request.direction,
            notify_clone,
        ));
        game_state.player_last_move_time[move_request.player_id] = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
    }

    notify.notified().await;

    let game_state = state.lock().await;
    let player = &game_state.zone.players[move_request.player_id];

    info!("processed /move request");

    Json(MoveResponse {
        my_new_coords: player.data.loc,
        rate_limited: false,
    })
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

#[post("/submit_round_1_key", format = "json", data = "<request>")]
async fn submit_round_1_key(
    state: &State<SharedState>,
    request: Json<SubmitRound1KeyRequest>,
) -> Json<()> {
    let mut game_state = state.lock().await;
    game_state.player_round_1_key[request.0.player_id] = Some(request.0.key);

    if game_state.player_round_1_key.iter().all(Option::is_some) {
        let round_1_keys: Vec<_> = game_state
            .player_round_1_key
            .iter()
            .flatten()
            .cloned()
            .collect();
        game_state.evaluator.aggregate_round_1_keys(&round_1_keys);
    }

    Json(())
}

#[post("/get_pk", format = "json")]
async fn get_pk(state: &State<SharedState>) -> Result<Json<GetPkResponse>, NotFound<String>> {
    if let Some(pk) = state.lock().await.evaluator.pk().cloned() {
        Ok(Json(GetPkResponse { pk }))
    } else {
        Err(NotFound("Public key not ready yet".to_string()))
    }
}

#[post("/submit_round_2_key", format = "json", data = "<request>")]
async fn submit_round_2_key(
    state: &State<SharedState>,
    request: Json<SubmitRound2KeyRequest>,
) -> Json<()> {
    let mut game_state = state.lock().await;
    game_state.player_round_2_key[request.0.player_id] = Some(request.0.key);

    if game_state.player_round_2_key.iter().all(Option::is_some) {
        let round_2_keys: Vec<_> = game_state
            .player_round_2_key
            .iter()
            .flatten()
            .cloned()
            .collect();
        game_state.evaluator.aggregate_round_2_keys(&round_2_keys);
    }

    Json(())
}

#[launch]
async fn rocket() -> _ {
    let shared_state: Arc<Mutex<GameState>> = Arc::new(Mutex::new(GameState {
        zone: Zone::new(64, 64), // 64x64 zone
        move_queue: VecDeque::new(),
        player_last_move_time: [0, 0, 0, 0],
        evaluator: PhantomEvaluator::new(PhantomParam::i_4p_60()),
        player_round_1_key: [None, None, None, None],
        player_round_2_key: [None, None, None, None],
    }));

    let state_clone = shared_state.clone();
    tokio::spawn(async move {
        process_moves(state_clone).await;
    });

    rocket::build()
        .manage(shared_state.clone())
        .mount(
            "/",
            routes![
                queue_move,
                get_cells,
                submit_round_1_key,
                get_pk,
                submit_round_2_key,
            ],
        )
        .attach(make_cors())
}
