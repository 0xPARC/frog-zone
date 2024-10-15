mod zone;

use rocket::http::Method;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use zone::{CellEncryptedData, Direction, Encrypted, EncryptedCoord, Zone};

#[macro_use]
extern crate rocket;

struct GameState {
    zone: Zone,
    move_queue: VecDeque<(usize, Encrypted<Direction>)>,
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
    let mut game_state = state.lock().await;
    game_state
        .move_queue
        .push_back((move_request.player_id, move_request.direction));

    info!("processed /get_cells request");

    Json(MoveResponse {
        my_new_coords: EncryptedCoord {
            x: Encrypted::<u8> { val: 0 },
            y: Encrypted::<u8> { val: 0 },
        },
    })
}

#[post("/get_cells", format = "json", data = "<request>")]
async fn get_cells(
    state: &State<SharedState>,
    request: Json<GetCellsRequest>,
) -> Json<GetCellsResponse> {
    let game_state = state.lock().await;
    let zone = &game_state.zone;

    let cell_data = zone
        .get_cells(request.player_id, request.coords.clone())
        .await;

    info!("processed /get_cells request");

    Json(GetCellsResponse { cell_data })
}

#[launch]
async fn rocket() -> _ {
    let shared_state: Arc<Mutex<GameState>> = Arc::new(Mutex::new(GameState {
        zone: Zone::new(64, 64), // 64x64 zone
        move_queue: VecDeque::new(),
    }));

    rocket::build()
        .manage(shared_state.clone())
        .mount("/", routes![queue_move, get_cells])
        .attach(make_cors())
}
