use itertools::Itertools;
use phantom::{PhantomEvaluator, PhantomParam};
use rocket::figment::{util::map, Figment};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{Config, State};
use server::bad_request;
use server::zone::{EncryptedCoord, ZoneDiff};
use server::{worker::*, zone::Zone};
use std::array::from_fn;
use std::env;
use std::sync::{Arc, LazyLock};
use tokio::sync::Mutex;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[macro_use]
extern crate rocket;

static PORT: LazyLock<u16> = LazyLock::new(|| {
    env::args()
        .nth(1)
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000)
});

struct WorkerState {
    zone: Option<Zone>,
    evaluator: PhantomEvaluator,
}

impl WorkerState {
    fn zone(&self) -> Result<&Zone, Custom<String>> {
        self.zone
            .as_ref()
            .ok_or_else(|| Custom(Status::BadRequest, "Worker is not init yet".to_string()))
    }

    fn apply_diff(&mut self, diff: ZoneDiff) -> Result<(), Custom<String>> {
        if let Some(zone) = &mut self.zone {
            zone.apply_diff(diff, &self.evaluator);
            Ok(())
        } else {
            Err(Custom(
                Status::BadRequest,
                "Worker is not init yet".to_string(),
            ))
        }
    }
}

type SharedState = Arc<Mutex<WorkerState>>;

#[post("/init", format = "json", data = "<request>")]
async fn init(state: &State<SharedState>, request: Json<InitRequest>) -> Json<InitResponse> {
    let InitRequest {
        zone_width,
        zone_height,
        zone_cts,
        pk,
        bs_key,
        rp_key,
    } = request.0;
    let mut app_state = state.lock().await;
    app_state.evaluator.set_pk(pk);
    app_state.evaluator.set_bs_key(bs_key);
    app_state.evaluator.set_rp_key(rp_key);
    app_state.zone = Some(Zone::from_cts(
        zone_width,
        zone_height,
        zone_cts,
        &app_state.evaluator,
    ));
    Json(InitResponse {})
}

#[post("/get_cells", format = "json", data = "<request>")]
async fn get_cells(
    state: &State<SharedState>,
    request: Json<RequestWithDiff<GetCellsRequest>>,
) -> Result<Json<GetCellsResponse>, Custom<String>> {
    let RequestWithDiff { request, diff } = request.0;
    let cell_data = {
        let mut worker_state = state.lock().await;
        worker_state.apply_diff(diff)?;
        let zone = worker_state.zone()?;

        let bits = worker_state.evaluator.unbatch(&request.coords);
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

        worker_state
            .evaluator
            .pack(cells.iter().flat_map(|cell| cell.bits()))
    };

    info!("processed /get_cells request");

    Ok(Json(GetCellsResponse { cell_data }))
}

#[post("/get_five_cells", format = "json", data = "<request>")]
async fn get_five_cells(
    state: &State<SharedState>,
    request: Json<RequestWithDiff<GetFiveCellsRequest>>,
) -> Result<Json<GetFiveCellsResponse>, Custom<String>> {
    let RequestWithDiff { request, diff } = request.0;
    let cell_data = {
        let mut worker_state = state.lock().await;
        worker_state.apply_diff(diff)?;
        let zone = worker_state.zone()?;

        let bits = worker_state.evaluator.unbatch(&request.coords);
        if bits.len() != 5 * 16 {
            return Err(bad_request("invalid coordinates"));
        }
        let mut bits = bits.into_iter();
        let coords = from_fn(|_| EncryptedCoord {
            x: from_fn(|_| bits.next().unwrap()),
            y: from_fn(|_| bits.next().unwrap()),
        });
        let cells = zone.get_five_cells(request.player_id, coords);

        worker_state
            .evaluator
            .pack(cells.iter().flat_map(|cell| cell.bits()))
    };

    info!("processed /get_five_cells request");

    Ok(Json(GetFiveCellsResponse { cell_data }))
}

#[post("/get_cross_cells", format = "json", data = "<request>")]
async fn get_cross_cells(
    state: &State<SharedState>,
    request: Json<RequestWithDiff<GetCrossCellsRequest>>,
) -> Result<Json<GetCrossCellsResponse>, Custom<String>> {
    let RequestWithDiff { request, diff } = request.0;
    let cell_data = {
        let mut worker_state = state.lock().await;
        worker_state.apply_diff(diff)?;
        let zone = worker_state.zone()?;

        let cells = zone.get_cross_cells(request.player_id);

        worker_state
            .evaluator
            .pack(cells.iter().flat_map(|cell| cell.bits()))
    };

    info!("processed /get_cross_cells request");

    Ok(Json(GetCrossCellsResponse { cell_data }))
}

#[post("/get_vertical_cells", format = "json", data = "<request>")]
async fn get_vertical_cells(
    state: &State<SharedState>,
    request: Json<RequestWithDiff<GetVerticalCellsRequest>>,
) -> Result<Json<GetVerticalCellsResponse>, Custom<String>> {
    let RequestWithDiff { request, diff } = request.0;
    let cell_data = {
        let mut worker_state = state.lock().await;
        worker_state.apply_diff(diff)?;
        let zone = worker_state.zone()?;

        let bits = worker_state.evaluator.unbatch(&request.coord);
        if bits.len() != 16 {
            return Err(bad_request("invalid coordinate"));
        }
        let mut bits = bits.into_iter();
        let coord = EncryptedCoord {
            x: from_fn(|_| bits.next().unwrap()),
            y: from_fn(|_| bits.next().unwrap()),
        };
        let cells = zone.get_vertical_cells(request.player_id, coord);

        worker_state
            .evaluator
            .pack(cells.iter().flat_map(|cell| cell.bits()))
    };

    info!("processed /get_vertical_cells request");

    Ok(Json(GetVerticalCellsResponse { cell_data }))
}

#[post("/get_horizontal_cells", format = "json", data = "<request>")]
async fn get_horizontal_cells(
    state: &State<SharedState>,
    request: Json<RequestWithDiff<GetHorizontalCellsRequest>>,
) -> Result<Json<GetHorizontalCellsResponse>, Custom<String>> {
    let RequestWithDiff { request, diff } = request.0;
    let cell_data = {
        let mut worker_state = state.lock().await;
        worker_state.apply_diff(diff)?;
        let zone = worker_state.zone()?;

        let bits = worker_state.evaluator.unbatch(&request.coord);
        if bits.len() != 16 {
            return Err(bad_request("invalid coordinate"));
        }
        let mut bits = bits.into_iter();
        let coord = EncryptedCoord {
            x: from_fn(|_| bits.next().unwrap()),
            y: from_fn(|_| bits.next().unwrap()),
        };
        let cells = zone.get_horizontal_cells(request.player_id, coord);

        worker_state
            .evaluator
            .pack(cells.iter().flat_map(|cell| cell.bits()))
    };

    info!("processed /get_horizontal_cells request");

    Ok(Json(GetHorizontalCellsResponse { cell_data }))
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let shared_state: Arc<Mutex<WorkerState>> = Arc::new(Mutex::new(WorkerState {
        zone: None, // 32x32 zone, will be initialized when /init is called.
        evaluator: PhantomEvaluator::new(PhantomParam::I_4P_40),
    }));

    rocket::Rocket::custom(
        Config::figment().merge(
            Figment::new()
                .join(("port", *PORT))
                .join(("limits", map!["json" => "700 MB"])),
        ),
    )
    .manage(shared_state.clone())
    .mount(
        "/",
        routes![
            init,
            get_cells,
            get_five_cells,
            get_cross_cells,
            get_vertical_cells,
            get_horizontal_cells,
        ],
    )
    .launch()
    .await
    .map(|_rocket| ()) // Convert `Result<Rocket<Ignite>, rocket::Error>` to `Result<(), rocket::Error>`
}
