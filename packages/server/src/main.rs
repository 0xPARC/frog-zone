use phantom::{PhantomEvaluator, PhantomParam, PhantomRound1Key, PhantomRound2Key};
use rocket::data::{Limits, ToByteUnit};
use rocket::figment::{util::map, Figment};
use rocket::futures::stream::FuturesUnordered;
use rocket::futures::TryStreamExt;
use rocket::http::{Method, Status};
use rocket::response::status::{Custom, NotFound};
use rocket::serde::{json::Json, Serialize};
use rocket::{Config, State};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use server::zone::{EncryptedDirection, Zone, ZoneDiff};
use server::{
    bad_request,
    client::*,
    worker::{self, *},
};
use std::collections::VecDeque;
use std::sync::{Arc, LazyLock};
use std::{env, mem};
use tokio::sync::{Mutex, Notify};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[macro_use]
extern crate rocket;

static WORKER_URIS: LazyLock<Vec<String>> = LazyLock::new(|| {
    env::args()
        .nth(1)
        .map(|p| p.split(",").map(ToString::to_string).collect())
        .unwrap_or_else(|| panic!("missing workers' uris"))
});

const MOVE_TIME_MILLIS: u64 = 500;
const GET_CELL_TIME_MILLIS: u64 = 140; // based on benchmark of 700ms for 5 cells
const GET_PLAYER_TIME_MILLIS: u64 = 140;
const MOVE_TIME_RATE_LIMIT_MILLIS: u64 = 3500;

struct GameState {
    zone: Option<Zone>,
    move_queue: VecDeque<(usize, EncryptedDirection, Arc<Notify>)>,
    player_last_move_time: [u64; 4],
    // Phantom
    evaluator: PhantomEvaluator,
    player_round_1_key: [Option<PhantomRound1Key>; 4],
    player_round_2_key: [Option<PhantomRound2Key>; 4],
    work_counter: usize,
    // For each worker, store flags indicating whether to sync players data or not.
    worker_diff: Vec<[bool; 4]>,
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

    fn next_worker_uri_and_diff(&mut self) -> Result<(&'static str, ZoneDiff), Custom<String>> {
        self.zone
            .as_ref()
            .map(|zone| {
                let next = self.work_counter % WORKER_URIS.len();
                self.work_counter += 1;
                (
                    WORKER_URIS[next].as_str(),
                    zone.cts_diff(mem::take(&mut self.worker_diff[next])),
                )
            })
            .ok_or_else(|| Custom(Status::BadRequest, "Game is not ready yet".to_string()))
    }
}

type SharedState = Arc<Mutex<GameState>>;

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
    let (worker_uri, diff) = state.lock().await.next_worker_uri_and_diff()?;
    let request = RequestWithDiff {
        request: request.0,
        diff,
    };
    worker::request(worker_uri, "/get_cells", request).await
}

#[post("/get_five_cells", format = "json", data = "<request>")]
async fn get_five_cells(
    state: &State<SharedState>,
    request: Json<GetFiveCellsRequest>,
) -> Result<Json<GetFiveCellsResponse>, Custom<String>> {
    let (worker_uri, diff) = state.lock().await.next_worker_uri_and_diff()?;
    let request = RequestWithDiff {
        request: request.0,
        diff,
    };
    worker::request(worker_uri, "/get_five_cells", request).await
}

#[post("/get_cross_cells", format = "json", data = "<request>")]
async fn get_cross_cells(
    state: &State<SharedState>,
    request: Json<GetCrossCellsRequest>,
) -> Result<Json<GetCrossCellsResponse>, Custom<String>> {
    let (worker_uri, diff) = state.lock().await.next_worker_uri_and_diff()?;
    let request = RequestWithDiff {
        request: request.0,
        diff,
    };
    worker::request(worker_uri, "/get_cross_cells", request).await
}

#[post("/get_vertical_cells", format = "json", data = "<request>")]
async fn get_vertical_cells(
    state: &State<SharedState>,
    request: Json<GetVerticalCellsRequest>,
) -> Result<Json<GetVerticalCellsResponse>, Custom<String>> {
    let (worker_uri, diff) = state.lock().await.next_worker_uri_and_diff()?;
    let request = RequestWithDiff {
        request: request.0,
        diff,
    };
    worker::request(worker_uri, "/get_vertical_cells", request).await
}

#[post("/get_horizontal_cells", format = "json", data = "<request>")]
async fn get_horizontal_cells(
    state: &State<SharedState>,
    request: Json<GetHorizontalCellsRequest>,
) -> Result<Json<GetHorizontalCellsResponse>, Custom<String>> {
    let (worker_uri, diff) = state.lock().await.next_worker_uri_and_diff()?;
    let request = RequestWithDiff {
        request: request.0,
        diff,
    };
    worker::request(worker_uri, "/get_horizontal_cells", request).await
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
            .pack(zone.get_player(request.player_id).bits())
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
            .bits(),
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

            // For each worker, mark the flag of `player_id` to be true.
            game_state
                .worker_diff
                .iter_mut()
                .for_each(|flag| flag[player_id] = true);
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
        game_state.evaluator.aggregate_round_1_keys(&round_1_keys);
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
) -> Result<Json<SubmitRound2KeyResponse>, Custom<String>> {
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
        game_state.zone = Some(Zone::new(64, 64, &game_state.evaluator));

        // Call /init to all workers
        let request = InitRequest {
            zone_width: 64,
            zone_height: 64,
            zone_cts: game_state.zone.as_ref().unwrap().cts(),
            pk: game_state.evaluator.pk().unwrap().clone(),
            bs_key: game_state.evaluator.bs_key().unwrap().clone(),
            rp_key: game_state.evaluator.rp_key().unwrap().clone(),
        };
        WORKER_URIS
            .iter()
            .map(|worker_uri| {
                worker::request::<_, InitResponse>(worker_uri, "/init", request.clone())
            })
            .collect::<FuturesUnordered<_>>()
            .try_collect::<Vec<_>>()
            .await?;
    }

    Ok(Json(SubmitRound2KeyResponse {}))
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
        evaluator: PhantomEvaluator::new(PhantomParam::I_4P_60),
        player_round_1_key: [None, None, None, None],
        player_round_2_key: [None, None, None, None],
        work_counter: 0,
        worker_diff: vec![Default::default(); WORKER_URIS.len()],
    }));

    let state_clone = shared_state.clone();
    tokio::spawn(async move {
        process_moves(state_clone).await;
    });

    let limits = Limits::default().limit("json", 750.mebibytes());

    let config = Config {
        port: 8000,
        address: std::net::IpAddr::V4("0.0.0.0".parse().unwrap()),
        limits,
        ..Config::default()
    };

    rocket::Rocket::custom(config)
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
