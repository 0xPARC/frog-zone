mod proxy;

use phantom::{PhantomParam, PhantomUser};
use rand::{rngs::StdRng, Rng, SeedableRng};
use reqwest::StatusCode;
use rocket::http::{Method, Status};
use rocket::response::status::Custom;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::Config;
use rocket::State;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

#[macro_use]
extern crate rocket;

struct AppState {
    user: PhantomUser,
}

impl AppState {
    fn new(player_id: usize) -> Self {
        let seed = StdRng::from_entropy().gen::<[u8; 32]>().to_vec();
        let user = PhantomUser::new(PhantomParam::I_4P_60, player_id, seed);
        Self { user }
    }
}

type SharedState = Arc<Mutex<AppState>>;

#[derive(Deserialize)]
struct SetIdRequest {
    player_id: usize,
}

#[derive(Serialize)]
struct SetIdResponse {
    player_id: usize,
}

#[derive(Deserialize)]
struct GetIdRequest {}

#[derive(Serialize)]
struct GetIdResponse {
    player_id: usize,
}

#[derive(Deserialize)]
struct GetPkRequest {}

#[derive(Serialize)]
struct GetPkResponse {}

#[derive(Deserialize)]
struct SubmitRound1KeyRequest {}

#[derive(Serialize)]
struct SubmitRound1KeyResponse {}

#[derive(Deserialize)]
struct SubmitRound2KeyRequest {}

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

#[post("/get_id", format = "json", data = "<_request>")]
async fn get_id(state: &State<SharedState>, _request: Json<GetIdRequest>) -> Json<GetIdResponse> {
    let app_state = state.lock().await;

    info!("processed /get_id request");
    Json(GetIdResponse {
        player_id: app_state.user.user_id(),
    })
}

#[post("/set_id", format = "json", data = "<request>")]
async fn set_id(state: &State<SharedState>, request: Json<SetIdRequest>) -> Json<SetIdResponse> {
    let mut app_state = state.lock().await;

    *app_state = AppState::new(request.player_id);

    info!("processed /set_id request");
    Json(SetIdResponse {
        player_id: app_state.user.user_id(),
    })
}

#[post("/submit_r1", format = "json", data = "<_request>")]
async fn submit_r1(
    state: &State<SharedState>,
    _request: Json<SubmitRound1KeyRequest>,
) -> Result<Json<SubmitRound1KeyResponse>, Custom<String>> {
    let app_state = state.lock().await;

    let post_data = proxy::SubmitRound1KeyRequest {
        player_id: app_state.user.user_id(),
        key: app_state.user.round_1_key_gen(),
    };

    let _: Json<proxy::SubmitRound1KeyResponse> = proxy::proxy("/submit_r1", post_data).await?;

    Ok(Json(SubmitRound1KeyResponse {}))
}

#[post("/get_pk", format = "json", data = "<_request>")]
async fn get_pk(
    state: &State<SharedState>,
    _request: Json<GetPkRequest>,
) -> Result<Json<GetPkResponse>, Custom<String>> {
    let mut app_state = state.lock().await;

    let response: proxy::GetPkResponse = proxy::proxy("/get_pk", proxy::GetPkRequest {}).await?.0;
    app_state.user.set_pk(response.pk.clone());

    Ok(Json(GetPkResponse {}))
}

#[post("/submit_r2", format = "json", data = "<_request>")]
async fn submit_r2(
    state: &State<SharedState>,
    _request: Json<SubmitRound2KeyRequest>,
) -> Result<Json<SubmitRound2KeyResponse>, Custom<String>> {
    let app_state = state.lock().await;

    if !app_state.user.has_pk() {
        return Err(bad_request("pk is not ready yet"));
    }

    let post_data = proxy::SubmitRound2KeyRequest {
        player_id: app_state.user.user_id(),
        key: app_state.user.round_2_key_gen(),
    };

    let _: Json<proxy::SubmitRound2KeyResponse> = proxy::proxy("/submit_r2", post_data).await?;

    Ok(Json(SubmitRound2KeyResponse {}))
}

fn bad_request(err: impl ToString) -> Custom<String> {
    custom(StatusCode::BAD_REQUEST, err)
}

fn internal_server_error(err: impl ToString) -> Custom<String> {
    custom(StatusCode::INTERNAL_SERVER_ERROR, err)
}

fn custom(stauts: StatusCode, err: impl ToString) -> Custom<String> {
    Custom(Status::from_code(stauts.as_u16()).unwrap(), err.to_string())
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Get the port from command line arguments
    let port = env::args()
        .nth(1)
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000);

    let player_id = env::args().nth(2).and_then(|p| p.parse().ok()).unwrap_or(0);
    let shared_state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState::new(player_id)));

    // Create a custom configuration
    let config = Config {
        port,
        ..Config::default()
    };

    rocket::custom(config)
        .manage(shared_state.clone())
        .mount("/", routes![get_id, set_id, get_pk, submit_r1, submit_r2])
        .attach(make_cors())
        .launch()
        .await
        .map(|_rocket| ()) // Convert `Result<Rocket<Ignite>, rocket::Error>` to `Result<(), rocket::Error>`
}
