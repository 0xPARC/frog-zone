mod zone;
use itertools::Itertools;
use phantom::{
    PhantomBatchedCt, PhantomEvaluator, PhantomPackedCt, PhantomParam, PhantomPk, PhantomRound1Key,
    PhantomRound2Key,
};
use rocket::figment::{util::map, Figment};
use rocket::http::{Method, Status};
use rocket::response::status::{Custom, NotFound};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{Config, State};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use std::array::from_fn;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use zone::{EncryptedCoord, EncryptedDirection, Zone};

use std::time::Duration;
use tokio::time;

#[macro_use]
extern crate rocket;

const MOVE_TIME_MILLIS: u64 = 500;
const GET_CELL_TIME_MILLIS: u64 = 140; // based on benchmark of 700ms for 5 cells
const GET_PLAYER_TIME_MILLIS: u64 = 140;
const MOVE_TIME_RATE_LIMIT_MILLIS: u64 = 3500;

struct GameState {
    zone: Option<Zone>,
    move_queue: VecDeque<(usize, EncryptedDirection, Arc<Notify>)>,
    player_last_move_time: [u64; 4],
    // Phantom
    evaluator: Arc<PhantomEvaluator>,
    player_round_1_key: [Option<PhantomRound1Key>; 4],
    player_round_2_key: [Option<PhantomRound2Key>; 4],
}

impl GameState {
    fn zone(&self) -> Result<&Zone, Custom<String>> {
        self.zone
            .as_ref()
            .ok_or_else(|| Custom(Status::BadRequest, "Game is not ready yet".to_string()))
    }

    fn zone_mut(&mut self) -> Result<&mut Zone, Custom<String>> {
        self.zone
            .as_mut()
            .ok_or_else(|| Custom(Status::BadRequest, "Game is not ready yet".to_string()))
    }
}

type SharedState = Arc<Mutex<GameState>>;

#[derive(Deserialize)]
struct GetCellsRequest {
    player_id: usize,
    coords: PhantomBatchedCt, // Vec<EncryptedCoord>
}

#[derive(Serialize, Clone)]
struct GetCellsResponse {
    cell_data: PhantomPackedCt, // Vec<CellEncryptedData>,
}

#[derive(Deserialize)]
struct GetFiveCellsRequest {
    player_id: usize,
    coords: PhantomBatchedCt, // [EncryptedCoord; 5]
}

#[derive(Serialize, Clone)]
struct GetFiveCellsResponse {
    cell_data: PhantomPackedCt, // [CellEncryptedData; 5],
}

#[derive(Deserialize)]
struct GetCrossCellsRequest {
    player_id: usize,
}

#[derive(Serialize, Clone)]
struct GetCrossCellsResponse {
    cell_data: PhantomPackedCt, // [CellEncryptedData; 5],
}

#[derive(Deserialize)]
struct GetVerticalCellsRequest {
    player_id: usize,
    coord: PhantomBatchedCt, // EncryptedCoord
}

#[derive(Serialize, Clone)]
struct GetVerticalCellsResponse {
    cell_data: PhantomPackedCt, // [CellEncryptedData; 5],
}

#[derive(Deserialize)]
struct GetHorizontalCellsRequest {
    player_id: usize,
    coord: PhantomBatchedCt, // EncryptedCoord
}

#[derive(Serialize, Clone)]
struct GetHorizontalCellsResponse {
    cell_data: PhantomPackedCt, // [CellEncryptedData; 5],
}

#[derive(Deserialize)]
struct GetPlayerRequest {
    player_id: usize,
}

#[derive(Serialize, Clone)]
struct GetPlayerResponse {
    player_data: PhantomPackedCt, // PlayerEncryptedData,
}

#[derive(Deserialize)]
struct MoveRequest {
    player_id: usize,
    direction: PhantomBatchedCt, // Encrypted<Direction>
}

#[derive(Serialize, Clone)]
struct MoveResponse {
    my_new_coords: Option<PhantomPackedCt>, // EncryptedCoord
    rate_limited: bool,
}

#[derive(Deserialize)]
struct SubmitRound1KeyRequest {
    player_id: usize,
    key: PhantomRound1Key,
}

#[derive(Serialize)]
struct SubmitRound1KeyResponse {}

#[derive(Deserialize)]
struct GetPkRequest {}

#[derive(Serialize)]
struct GetPkResponse {
    pk: PhantomPk,
}

#[derive(Deserialize)]
struct SubmitRound2KeyRequest {
    player_id: usize,
    key: PhantomRound2Key,
}

#[derive(Serialize)]
struct SubmitRound2KeyResponse {}

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
) -> Result<Json<GetCellsResponse>, Custom<String>> {
    let cell_data = {
        let game_state = state.lock().await;
        let zone = game_state.zone()?;

        let bits = game_state.evaluator.unbatch(&request.coords);
        if bits.len() % 16 != 0 {
            return Err(bad_request("invalid coordinates"));
        }
        let coords = bits
            .into_iter()
            .chunks(16)
            .into_iter()
            .map(|mut chunk| EncryptedCoord {
                x: from_fn(|_| chunk.next().unwrap()),
                y: from_fn(|_| chunk.next().unwrap()),
            })
            .collect();
        let cells = zone.get_cells(request.player_id, coords);

        game_state
            .evaluator
            .pack(cells.iter().flat_map(|cell| cell.cts()))
    };

    info!("processed /get_cells request");

    Ok(Json(GetCellsResponse { cell_data }))
}

#[post("/get_five_cells", format = "json", data = "<request>")]
async fn get_five_cells(
    state: &State<SharedState>,
    request: Json<GetFiveCellsRequest>,
) -> Result<Json<GetFiveCellsResponse>, Custom<String>> {
    let cell_data = {
        let game_state = state.lock().await;
        let zone = game_state.zone()?;

        let bits = game_state.evaluator.unbatch(&request.coords);
        if bits.len() != 5 * 16 {
            return Err(bad_request("invalid coordinates"));
        }
        let mut bits = bits.into_iter();
        let coords = from_fn(|_| EncryptedCoord {
            x: from_fn(|_| bits.next().unwrap()),
            y: from_fn(|_| bits.next().unwrap()),
        });
        let cells = zone.get_five_cells(request.player_id, coords);

        game_state
            .evaluator
            .pack(cells.iter().flat_map(|cell| cell.cts()))
    };

    info!("processed /get_five_cells request");

    Ok(Json(GetFiveCellsResponse { cell_data }))
}

#[post("/get_cross_cells", format = "json", data = "<request>")]
async fn get_cross_cells(
    state: &State<SharedState>,
    request: Json<GetCrossCellsRequest>,
) -> Result<Json<GetCrossCellsResponse>, Custom<String>> {
    let cell_data = {
        let game_state = state.lock().await;
        let zone = game_state.zone()?;

        let cells = zone.get_cross_cells(request.player_id);

        game_state
            .evaluator
            .pack(cells.iter().flat_map(|cell| cell.cts()))
    };

    info!("processed /get_cross_cells request");

    Ok(Json(GetCrossCellsResponse { cell_data }))
}

#[post("/get_vertical_cells", format = "json", data = "<request>")]
async fn get_vertical_cells(
    state: &State<SharedState>,
    request: Json<GetVerticalCellsRequest>,
) -> Result<Json<GetVerticalCellsResponse>, Custom<String>> {
    let cell_data = {
        let game_state = state.lock().await;
        let zone = game_state.zone()?;

        let bits = game_state.evaluator.unbatch(&request.coord);
        if bits.len() != 16 {
            return Err(bad_request("invalid coordinate"));
        }
        let mut bits = bits.into_iter();
        let coord = EncryptedCoord {
            x: from_fn(|_| bits.next().unwrap()),
            y: from_fn(|_| bits.next().unwrap()),
        };
        let cells = zone.get_vertical_cells(request.player_id, coord);

        game_state
            .evaluator
            .pack(cells.iter().flat_map(|cell| cell.cts()))
    };

    info!("processed /get_vertical_cells request");

    Ok(Json(GetVerticalCellsResponse { cell_data }))
}

#[post("/get_horizontal_cells", format = "json", data = "<request>")]
async fn get_horizontal_cells(
    state: &State<SharedState>,
    request: Json<GetHorizontalCellsRequest>,
) -> Result<Json<GetHorizontalCellsResponse>, Custom<String>> {
    let cell_data = {
        let game_state = state.lock().await;
        let zone = game_state.zone()?;

        let bits = game_state.evaluator.unbatch(&request.coord);
        if bits.len() != 16 {
            return Err(bad_request("invalid coordinate"));
        }
        let mut bits = bits.into_iter();
        let coord = EncryptedCoord {
            x: from_fn(|_| bits.next().unwrap()),
            y: from_fn(|_| bits.next().unwrap()),
        };
        let cells = zone.get_horizontal_cells(request.player_id, coord);

        game_state
            .evaluator
            .pack(cells.iter().flat_map(|cell| cell.cts()))
    };

    info!("processed /get_horizontal_cells request");

    Ok(Json(GetHorizontalCellsResponse { cell_data }))
}

#[post("/get_player", format = "json", data = "<request>")]
async fn get_player(
    state: &State<SharedState>,
    request: Json<GetPlayerRequest>,
) -> Result<Json<GetPlayerResponse>, Custom<String>> {
    let player_response = {
        let game_state = state.lock().await;
        let zone = game_state.zone()?;
        game_state
            .evaluator
            .pack(zone.get_player(request.player_id).cts())
    };

    info!("processed /get_player request");

    Ok(Json(GetPlayerResponse {
        player_data: player_response,
    }))
}

#[post("/move", format = "json", data = "<move_request>")]
async fn queue_move(
    state: &State<SharedState>,
    move_request: Json<MoveRequest>,
) -> Result<Json<MoveResponse>, Custom<String>> {
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
        return Ok(Json(MoveResponse {
            my_new_coords: None,
            rate_limited: true,
        }));
    }

    let notify = Arc::new(Notify::new());
    let notify_clone = notify.clone();

    {
        let mut game_state = state.lock().await;
        let direction = game_state
            .evaluator
            .unbatch(&move_request.direction)
            .try_into()
            .map_err(|_| bad_request("invalid direction"))?;
        game_state
            .move_queue
            .push_back((move_request.player_id, direction, notify_clone));
        game_state.player_last_move_time[move_request.player_id] = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
    }

    notify.notified().await;

    let game_state = state.lock().await;
    let my_new_coords = game_state.evaluator.pack(
        game_state.zone()?.players[move_request.player_id]
            .data
            .loc
            .cts(),
    );

    info!("processed /move request");

    Ok(Json(MoveResponse {
        my_new_coords: Some(my_new_coords),
        rate_limited: false,
    }))
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

        {
            let mut game_state = state.lock().await;
            let zone = game_state.zone_mut().unwrap();
            let start = std::time::Instant::now();
            zone.move_player(player_id, direction);
            info!("zone.move_player takes: {:?}", start.elapsed());
        }

        notify.notify_one();
    }
}

#[post("/submit_r1", format = "json", data = "<request>")]
async fn submit_r1(
    state: &State<SharedState>,
    request: Json<SubmitRound1KeyRequest>,
) -> Json<SubmitRound1KeyResponse> {
    let mut game_state = state.lock().await;
    game_state.player_round_1_key[request.0.player_id] = Some(request.0.key);

    if game_state.player_round_1_key.iter().all(Option::is_some) {
        let round_1_keys: Vec<_> = game_state
            .player_round_1_key
            .iter()
            .flatten()
            .cloned()
            .collect();
        Arc::make_mut(&mut game_state.evaluator).aggregate_round_1_keys(&round_1_keys);
    }

    Json(SubmitRound1KeyResponse {})
}

#[post("/get_pk", format = "json", data = "<_request>")]
async fn get_pk(
    state: &State<SharedState>,
    _request: Json<GetPkRequest>,
) -> Result<Json<GetPkResponse>, NotFound<String>> {
    if let Some(pk) = state.lock().await.evaluator.pk().cloned() {
        Ok(Json(GetPkResponse { pk }))
    } else {
        Err(NotFound("Public key not ready yet".to_string()))
    }
}

#[post("/submit_r2", format = "json", data = "<request>")]
async fn submit_r2(
    state: &State<SharedState>,
    request: Json<SubmitRound2KeyRequest>,
) -> Json<SubmitRound2KeyResponse> {
    let mut game_state = state.lock().await;
    game_state.player_round_2_key[request.0.player_id] = Some(request.0.key);

    if game_state.player_round_2_key.iter().all(Option::is_some) {
        let round_2_keys: Vec<_> = game_state
            .player_round_2_key
            .iter()
            .flatten()
            .cloned()
            .collect();
        Arc::make_mut(&mut game_state.evaluator).aggregate_round_2_keys(&round_2_keys);
        game_state.zone = Some(zone::Zone::new(64, 64, &game_state.evaluator));
    }

    Json(SubmitRound2KeyResponse {})
}

fn bad_request(err: impl ToString) -> Custom<String> {
    custom(Status::BadRequest, err)
}

fn custom(status: Status, err: impl ToString) -> Custom<String> {
    Custom(status, err.to_string())
}

#[launch]
async fn rocket() -> _ {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let shared_state: Arc<Mutex<GameState>> = Arc::new(Mutex::new(GameState {
        zone: None, // 64x64 zone, will be initialized when keygen is finished.
        move_queue: VecDeque::new(),
        player_last_move_time: [0, 0, 0, 0],
        evaluator: Arc::new(PhantomEvaluator::new(PhantomParam::I_4P_60)),
        player_round_1_key: [None, None, None, None],
        player_round_2_key: [None, None, None, None],
    }));

    let state_clone = shared_state.clone();
    tokio::spawn(async move {
        process_moves(state_clone).await;
    });

    rocket::Rocket::custom(
        Config::figment().merge(Figment::new().join(("limits", map!["json" => "700 MB"]))),
    )
    .manage(shared_state.clone())
    .mount(
        "/",
        routes![
            queue_move,
            get_cells,
            get_five_cells,
            get_cross_cells,
            get_vertical_cells,
            get_horizontal_cells,
            get_player,
            submit_r1,
            get_pk,
            submit_r2,
        ],
    )
    .attach(make_cors())
}
