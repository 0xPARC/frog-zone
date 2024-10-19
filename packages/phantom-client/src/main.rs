use phantom::{PhantomParam, PhantomPk, PhantomRound1Key, PhantomRound2Key, PhantomUser};
use rand::{rngs::StdRng, Rng, SeedableRng};
use reqwest;
use rocket::http::Method;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::Config;
use rocket::State;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use serde_json;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

#[macro_use]
extern crate rocket;

struct AppState {
    player_id: u8,
}

type SharedState = Arc<Mutex<AppState>>;

#[derive(Deserialize)]
struct SetIdRequest {
    player_id: u8,
}

#[derive(Deserialize)]
struct GetIdRequest {}

#[derive(Serialize, Clone)]
struct SetIdResponse {
    player_id: u8,
}

#[derive(Serialize, Clone)]
struct GetIdResponse {
    player_id: u8,
}

// copied from server/src/main.rs
#[derive(Serialize, Deserialize)]
struct SubmitRound1KeyRequest {
    player_id: usize,
    key: PhantomRound1Key,
}

// copied from server/src/main.rs
#[derive(Serialize, Clone)]
struct GetPkResponse {
    pk: PhantomPk,
}

// copied from server/src/main.rs
#[derive(Serialize, Deserialize)]
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

#[post("/get_id", format = "json", data = "<_request>")]
async fn get_id(state: &State<SharedState>, _request: Json<GetIdRequest>) -> Json<GetIdResponse> {
    let app_state = state.lock().await;

    info!("processed /get_id request");
    Json(GetIdResponse {
        player_id: app_state.player_id,
    })
}

#[post("/set_id", format = "json", data = "<request>")]
async fn set_id(state: &State<SharedState>, request: Json<SetIdRequest>) -> Json<SetIdResponse> {
    let mut app_state = state.lock().await;

    app_state.player_id = request.player_id;

    info!("processed /set_id request");
    Json(SetIdResponse {
        player_id: app_state.player_id,
    })
}

async fn submit_r1(player_id: u8) -> Result<(), reqwest::Error> {
    // Create a client
    let client = reqwest::Client::new();

    let param = PhantomParam::i_4p_60();
    let seed = StdRng::from_entropy().gen::<[u8; 32]>().to_vec();
    let user = PhantomUser::new(param, player_id as usize, seed);

    // Prepare the data to send
    let post_data = SubmitRound1KeyRequest {
        player_id: player_id as usize,
        key: user.round_1_key_gen(),
    };

    let json = serde_json::to_string_pretty(&post_data).unwrap();
    println!("Post data: {}", json);

    // Send the POST request
    let response = client
        .post("http://localhost:8000/submit_round_1_key")
        .json(&post_data)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        let body = response.text().await?;
        println!("Response: {}", body);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Get the port from command line arguments
    let port = env::args()
        .nth(1)
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000);

    let player_id = env::args().nth(2).and_then(|p| p.parse().ok()).unwrap_or(0);

    let shared_state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState { player_id }));

    submit_r1(player_id).await;

    // Create a custom configuration
    let config = Config {
        port,
        ..Config::default()
    };

    rocket::custom(config)
        .manage(shared_state.clone())
        .mount("/", routes![get_id, set_id])
        .attach(make_cors())
        .launch()
        .await
        .map(|_rocket| ()) // Convert `Result<Rocket<Ignite>, rocket::Error>` to `Result<(), rocket::Error>`
}
