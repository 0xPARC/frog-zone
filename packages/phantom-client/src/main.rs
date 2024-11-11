mod proxy;

use itertools::{chain, Itertools};
use phantom::{PhantomPackedCt, PhantomPackedCtDecShare, PhantomParam, PhantomUser};
use rand::thread_rng;
use rand::{rngs::StdRng, Rng, SeedableRng};
use reqwest::StatusCode;
use rocket::futures::stream::FuturesUnordered;
use rocket::futures::TryStreamExt;
use rocket::http::{Method, Status};
use rocket::response::status::Custom;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::Config;
use rocket::State;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use server::client::{Direction, EntityType};
use server::mock_zone::{CellEncryptedData, MockEncryptedCoord};
use std::array::from_fn;
use std::env;
use std::iter::repeat_with;
use std::sync::{Arc, LazyLock};
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time;
use tracing::info;

#[macro_use]
extern crate rocket;

const GET_CELL_MOCK_TIME_MILLIS: u64 = 140; // based on benchmark of 700ms for 5 cells
const MOVE_MOCK_TIME_MILLIS: u64 = 750;

static PORT: LazyLock<u16> = LazyLock::new(|| {
    env::args()
        .nth(1)
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000)
});

static PLAYER_ID: LazyLock<usize> =
    LazyLock::new(|| env::args().nth(2).and_then(|p| p.parse().ok()).unwrap_or(0));

static SERVER_URI: LazyLock<String> = LazyLock::new(|| {
    env::args()
        .nth(3)
        .map(|p| p.to_string())
        .unwrap_or_else(|| panic!("missing server's uri"))
});

static OTHER_PLAYER_URIS: LazyLock<[String; 3]> = LazyLock::new(|| {
    env::args()
        .nth(4)
        .and_then(|p| {
            let mut p = p.split(",");
            Some([
                p.next()?.to_string(),
                p.next()?.to_string(),
                p.next()?.to_string(),
            ])
        })
        .unwrap_or_else(|| panic!("missing other 3 players' uris"))
});

struct AppState {
    user: PhantomUser,
    player_coord: Coord,
}

impl AppState {
    fn new(player_id: usize) -> Self {
        let seed = StdRng::from_entropy().gen::<[u8; 32]>().to_vec();
        let user = PhantomUser::new(PhantomParam::I_4P_40, player_id, seed);
        Self {
            user,
            player_coord: Coord { x: 0, y: 0 },
        }
    }

    fn decrypt(&self, ct: &PhantomPackedCt, dec_shares: [PhantomPackedCtDecShare; 3]) -> Vec<bool> {
        self.user.aggregate_dec_shares(
            ct,
            chain![dec_shares, [self.user.decrypt_share(ct)]].collect(),
        )
    }
}

type SharedState = Arc<Mutex<AppState>>;

fn to_le_bits(value: u8) -> impl Iterator<Item = bool> {
    (0..8).map(move |i| (value >> i) & 1 == 1)
}

fn try_from_le_bits<const N: usize>(
    bits: &mut impl Iterator<Item = bool>,
) -> Result<u8, Custom<String>> {
    (0..N)
        .try_fold(0, |value, i| Some(value ^ ((bits.next()? as u8) << i)))
        .ok_or(internal_server_error(""))
}

fn cell_try_from_le_bits(
    bits: &mut impl Iterator<Item = bool>,
) -> Result<CellData, Custom<String>> {
    Ok(CellData {
        entity_type: match try_from_le_bits::<3>(bits)? {
            0 => EntityType::Invalid,
            1 => EntityType::Player,
            2 => EntityType::Item,
            3 => EntityType::Monster,
            4 => EntityType::None,
            _ => return Err(internal_server_error("")),
        },
        entity_id: try_from_le_bits::<8>(bits)?,
        hp: try_from_le_bits::<8>(bits)?,
        atk: try_from_le_bits::<8>(bits)?,
        points: try_from_le_bits::<8>(bits)?,
    })
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Coord {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerData {
    pub loc: Coord,
    pub hp: u8,
    pub atk: u8,
    pub points: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CellData {
    pub entity_type: EntityType,
    pub entity_id: u8,
    pub hp: u8,
    pub atk: u8,
    pub points: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetCellsRequest {
    coords: Vec<Coord>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetCellsResponse {
    cell_data: Vec<CellData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetFiveCellsRequest {
    coords: [Coord; 5],
}

#[derive(Debug, Serialize, Deserialize)]
struct GetFiveCellsResponse {
    cell_data: [CellData; 5],
}

#[derive(Debug, Serialize, Deserialize)]
struct GetCrossCellsRequest {}

#[derive(Debug, Serialize, Deserialize)]
struct GetCrossCellsResponse {
    cell_data: [CellData; 5], // [(x,y), (x,y+1), (x,y-1), (x+1,y), (x-1,y)]
}

#[derive(Debug, Serialize, Deserialize)]
struct GetVerticalCellsRequest {
    center_coord: Coord,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetVerticalCellsResponse {
    cell_data: [CellData; 5], // [(x,y-2), (x,y-1), (x,y), (x,y+1), (x,y+2)]
}

#[derive(Debug, Serialize, Deserialize)]
struct GetHorizontalCellsRequest {
    center_coord: Coord,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetHorizontalCellsResponse {
    cell_data: [CellData; 5], // [(x-2,y), (x-1,y), (x,y), (x+1,y), (x+2,y)]
}

#[derive(Debug, Serialize, Deserialize)]
struct GetPlayerRequest {}

#[derive(Debug, Serialize, Deserialize)]
struct GetPlayerResponse {
    player_data: PlayerData,
}

#[derive(Debug, Serialize, Deserialize)]
struct MoveRequest {
    direction: Direction,
}

#[derive(Debug, Serialize, Deserialize)]
struct MoveResponse {
    my_new_coords: Option<Coord>,
    rate_limited: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResetGameRequest {
    player_id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResetGameResponse {}

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

#[derive(Serialize, Deserialize)]
struct GetDecShareRequest {
    ct: PhantomPackedCt,
}

#[derive(Serialize, Deserialize)]
struct GetDecShareResponse {
    dec_share: PhantomPackedCtDecShare,
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

fn mock_encrypt_coord(coord: Coord) -> MockEncryptedCoord {
    return MockEncryptedCoord {
        x: coord.x,
        y: coord.y,
    };
}

fn mock_decrypt_coord(coord: MockEncryptedCoord) -> Coord {
    return Coord {
        x: coord.x,
        y: coord.y,
    };
}

fn mock_decrypt_cell(cell: CellEncryptedData) -> CellData {
    return CellData {
        entity_type: cell.entity_type,
        entity_id: cell.entity_id,
        atk: cell.atk,
        hp: cell.hp,
        points: cell.points,
    };
}

#[post("/reset_game", format = "json", data = "<_request>")]
async fn reset_game(
    state: &State<SharedState>,
    _request: Json<ResetGameRequest>,
) -> Result<Json<ResetGameResponse>, Custom<String>> {
    let app_state = state.lock().await;

    let post_data = proxy::ResetGameRequest {
        player_id: app_state.user.user_id(),
    };

    let proxy::ResetGameResponse {} = proxy::proxy(&*SERVER_URI, "/reset_game", post_data)
        .await?
        .0;

    Ok(Json(ResetGameResponse {}))
}

#[post("/mock_get_cells", format = "json", data = "<request>")]
async fn mock_get_cells(
    state: &State<SharedState>,
    request: Json<GetCellsRequest>,
) -> Result<Json<GetCellsResponse>, Custom<String>> {
    let app_state = state.lock().await;

    let post_data = proxy::MockGetCellsRequest {
        player_id: app_state.user.user_id(),
        coords: request
            .coords
            .iter()
            .map(|c| mock_encrypt_coord(c.clone()))
            .collect(),
    };

    let proxy::MockGetCellsResponse { cell_data } =
        proxy::proxy(&*SERVER_URI, "/mock_get_cells", post_data)
            .await?
            .0;

    let len = request.coords.len();
    time::sleep(Duration::from_millis(
        GET_CELL_MOCK_TIME_MILLIS * (len as u64),
    ))
    .await;

    Ok(Json(GetCellsResponse {
        cell_data: cell_data
            .iter()
            .map(|c| mock_decrypt_cell(c.clone()))
            .collect(),
    }))
}

#[post("/get_cells", format = "json", data = "<request>")]
async fn get_cells(
    state: &State<SharedState>,
    request: Json<GetCellsRequest>,
) -> Result<Json<GetCellsResponse>, Custom<String>> {
    let post_data = {
        let app_state = state.lock().await;

        let coords = app_state.user.batched_pk_encrypt(
            request
                .coords
                .iter()
                .flat_map(|coord| chain![to_le_bits(coord.x), to_le_bits(coord.y)]),
        );

        proxy::GetCellsRequest {
            player_id: app_state.user.user_id(),
            coords,
        }
    };

    let proxy::GetCellsResponse { cell_data } =
        proxy::proxy(&*SERVER_URI, "/get_cells", post_data).await?.0;

    let dec_shares = get_dec_shares(&cell_data).await?;
    let mut bits = state
        .lock()
        .await
        .decrypt(&cell_data, dec_shares)
        .into_iter();
    let cell_data = repeat_with(|| cell_try_from_le_bits(&mut bits))
        .take(cell_data.n() / 27)
        .try_collect()?;

    Ok(Json(GetCellsResponse { cell_data }))
}

#[post("/get_five_cells", format = "json", data = "<request>")]
async fn get_five_cells(
    state: &State<SharedState>,
    request: Json<GetFiveCellsRequest>,
) -> Result<Json<GetFiveCellsResponse>, Custom<String>> {
    let post_data = {
        let app_state = state.lock().await;

        let coords = app_state.user.batched_pk_encrypt(
            request
                .coords
                .iter()
                .flat_map(|coord| chain![to_le_bits(coord.x), to_le_bits(coord.y)]),
        );

        proxy::GetFiveCellsRequest {
            player_id: app_state.user.user_id(),
            coords,
        }
    };

    let proxy::GetFiveCellsResponse { cell_data } =
        proxy::proxy(&*SERVER_URI, "/get_five_cells", post_data)
            .await?
            .0;

    let dec_shares = get_dec_shares(&cell_data).await?;
    let mut bits = state
        .lock()
        .await
        .decrypt(&cell_data, dec_shares)
        .into_iter();
    let cell_data = [
        cell_try_from_le_bits(&mut bits)?,
        cell_try_from_le_bits(&mut bits)?,
        cell_try_from_le_bits(&mut bits)?,
        cell_try_from_le_bits(&mut bits)?,
        cell_try_from_le_bits(&mut bits)?,
    ];

    Ok(Json(GetFiveCellsResponse { cell_data }))
}

#[post("/get_cross_cells", format = "json", data = "<_request>")]
async fn get_cross_cells(
    state: &State<SharedState>,
    _request: Json<GetCrossCellsRequest>,
) -> Result<Json<GetCrossCellsResponse>, Custom<String>> {
    let post_data = {
        let app_state = state.lock().await;

        proxy::GetCrossCellsRequest {
            player_id: app_state.user.user_id(),
        }
    };

    let proxy::GetCrossCellsResponse { cell_data } =
        proxy::proxy(&*SERVER_URI, "/get_cross_cells", post_data)
            .await?
            .0;

    let dec_shares = get_dec_shares(&cell_data).await?;
    let mut bits = state
        .lock()
        .await
        .decrypt(&cell_data, dec_shares)
        .into_iter();
    let cell_data = [
        cell_try_from_le_bits(&mut bits)?,
        cell_try_from_le_bits(&mut bits)?,
        cell_try_from_le_bits(&mut bits)?,
        cell_try_from_le_bits(&mut bits)?,
        cell_try_from_le_bits(&mut bits)?,
    ];

    Ok(Json(GetCrossCellsResponse { cell_data }))
}

#[post("/get_vertical_cells", format = "json", data = "<request>")]
async fn get_vertical_cells(
    state: &State<SharedState>,
    request: Json<GetVerticalCellsRequest>,
) -> Result<Json<GetVerticalCellsResponse>, Custom<String>> {
    let post_data = {
        let app_state = state.lock().await;

        let coord = app_state.user.batched_pk_encrypt(chain![
            to_le_bits(request.center_coord.x),
            to_le_bits(request.center_coord.y)
        ]);

        proxy::GetVerticalCellsRequest {
            player_id: app_state.user.user_id(),
            coord,
        }
    };

    let proxy::GetVerticalCellsResponse { cell_data } =
        proxy::proxy(&*SERVER_URI, "/get_vertical_cells", post_data)
            .await?
            .0;

    let dec_shares = get_dec_shares(&cell_data).await?;
    let mut bits = state
        .lock()
        .await
        .decrypt(&cell_data, dec_shares)
        .into_iter();
    let cell_data = [
        cell_try_from_le_bits(&mut bits)?,
        cell_try_from_le_bits(&mut bits)?,
        cell_try_from_le_bits(&mut bits)?,
        cell_try_from_le_bits(&mut bits)?,
        cell_try_from_le_bits(&mut bits)?,
    ];

    Ok(Json(GetVerticalCellsResponse { cell_data }))
}

#[post("/get_horizontal_cells", format = "json", data = "<request>")]
async fn get_horizontal_cells(
    state: &State<SharedState>,
    request: Json<GetHorizontalCellsRequest>,
) -> Result<Json<GetHorizontalCellsResponse>, Custom<String>> {
    let post_data = {
        let app_state = state.lock().await;

        let coord = app_state.user.batched_pk_encrypt(chain![
            to_le_bits(request.center_coord.x),
            to_le_bits(request.center_coord.y)
        ]);

        proxy::GetHorizontalCellsRequest {
            player_id: app_state.user.user_id(),
            coord,
        }
    };

    let proxy::GetHorizontalCellsResponse { cell_data } =
        proxy::proxy(&*SERVER_URI, "/get_horizontal_cells", post_data)
            .await?
            .0;

    let dec_shares = get_dec_shares(&cell_data).await?;
    let mut bits = state
        .lock()
        .await
        .decrypt(&cell_data, dec_shares)
        .into_iter();
    let cell_data = [
        cell_try_from_le_bits(&mut bits)?,
        cell_try_from_le_bits(&mut bits)?,
        cell_try_from_le_bits(&mut bits)?,
        cell_try_from_le_bits(&mut bits)?,
        cell_try_from_le_bits(&mut bits)?,
    ];

    Ok(Json(GetHorizontalCellsResponse { cell_data }))
}

#[post("/mock_get_player", format = "json", data = "<_request>")]
async fn mock_get_player(
    state: &State<SharedState>,
    _request: Json<GetPlayerRequest>,
) -> Result<Json<GetPlayerResponse>, Custom<String>> {
    let post_data = {
        let app_state = state.lock().await;

        proxy::MockGetPlayerRequest {
            player_id: app_state.user.user_id(),
        }
    };

    let proxy::MockGetPlayerResponse { player_data } =
        proxy::proxy(&*SERVER_URI, "/mock_get_player", post_data)
            .await?
            .0;

    let player_data = PlayerData {
        loc: mock_decrypt_coord(player_data.loc),
        hp: player_data.hp,
        atk: player_data.atk,
        points: player_data.points,
    };

    state.lock().await.player_coord = player_data.loc;

    Ok(Json(GetPlayerResponse { player_data }))
}

#[post("/get_player", format = "json", data = "<_request>")]
async fn get_player(
    state: &State<SharedState>,
    _request: Json<GetPlayerRequest>,
) -> Result<Json<GetPlayerResponse>, Custom<String>> {
    let post_data = {
        let app_state = state.lock().await;

        proxy::GetPlayerRequest {
            player_id: app_state.user.user_id(),
        }
    };

    let proxy::GetPlayerResponse { player_data } =
        proxy::proxy(&*SERVER_URI, "/get_player", post_data)
            .await?
            .0;

    let dec_shares = get_dec_shares(&player_data).await?;
    let mut bits = state
        .lock()
        .await
        .decrypt(&player_data, dec_shares)
        .into_iter();
    let player_data = PlayerData {
        loc: Coord {
            x: try_from_le_bits::<8>(&mut bits)?,
            y: try_from_le_bits::<8>(&mut bits)?,
        },
        hp: try_from_le_bits::<8>(&mut bits)?,
        atk: try_from_le_bits::<8>(&mut bits)?,
        points: try_from_le_bits::<8>(&mut bits)?,
    };

    state.lock().await.player_coord = player_data.loc;

    Ok(Json(GetPlayerResponse { player_data }))
}

#[post("/mock_move", format = "json", data = "<request>")]
async fn mock_move(
    state: &State<SharedState>,
    request: Json<MoveRequest>,
) -> Result<Json<MoveResponse>, Custom<String>> {
    let post_data = {
        let app_state = state.lock().await;

        let random_input = thread_rng().gen();
        proxy::MockMoveRequest {
            player_id: app_state.user.user_id(),
            direction_and_random_input: (request.direction, random_input),
        }
    };

    let proxy::MockMoveResponse {
        my_new_coords,
        rate_limited,
    } = proxy::proxy(&*SERVER_URI, "/mock_move", post_data).await?.0;

    let my_new_coords = if let Some(my_new_coords) = my_new_coords {
        let mut app_state = state.lock().await;
        let coord = mock_decrypt_coord(my_new_coords);
        app_state.player_coord = coord;
        Some(coord)
    } else {
        None
    };

    time::sleep(Duration::from_millis(MOVE_MOCK_TIME_MILLIS)).await;

    Ok(Json(MoveResponse {
        my_new_coords,
        rate_limited,
    }))
}

#[post("/move", format = "json", data = "<request>")]
async fn queue_move(
    state: &State<SharedState>,
    request: Json<MoveRequest>,
) -> Result<Json<MoveResponse>, Custom<String>> {
    let post_data = {
        let app_state = state.lock().await;

        let direction = match request.direction {
            Direction::Up => [false, false],
            Direction::Down => [true, false],
            Direction::Left => [false, true],
            Direction::Right => [true, true],
        };
        let random_input = {
            let v: u8 = thread_rng().gen();
            from_fn::<_, 8, _>(|i| (v >> i) & 1 == 1)
        };
        let direction_and_random_input = app_state
            .user
            .batched_pk_encrypt(chain![direction, random_input]);

        proxy::MoveRequest {
            player_id: app_state.user.user_id(),
            direction_and_random_input,
        }
    };

    let proxy::MoveResponse {
        my_new_coords,
        rate_limited,
    } = proxy::proxy(&*SERVER_URI, "/move", post_data).await?.0;

    let my_new_coords = if let Some(my_new_coords) = my_new_coords {
        let dec_shares = get_dec_shares(&my_new_coords).await?;
        let mut app_state = state.lock().await;
        let mut bits = app_state.decrypt(&my_new_coords, dec_shares).into_iter();
        let coord = Coord {
            x: try_from_le_bits::<8>(&mut bits)?,
            y: try_from_le_bits::<8>(&mut bits)?,
        };
        app_state.player_coord = coord;
        Some(coord)
    } else {
        None
    };

    Ok(Json(MoveResponse {
        my_new_coords,
        rate_limited,
    }))
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

    let _: Json<proxy::SubmitRound1KeyResponse> =
        proxy::proxy(&*SERVER_URI, "/submit_r1", post_data).await?;

    Ok(Json(SubmitRound1KeyResponse {}))
}

#[post("/get_pk", format = "json", data = "<_request>")]
async fn get_pk(
    state: &State<SharedState>,
    _request: Json<GetPkRequest>,
) -> Result<Json<GetPkResponse>, Custom<String>> {
    let mut app_state = state.lock().await;

    let response: proxy::GetPkResponse =
        proxy::proxy(&*SERVER_URI, "/get_pk", proxy::GetPkRequest {})
            .await?
            .0;
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

    let _: Json<proxy::SubmitRound2KeyResponse> =
        proxy::proxy(&*SERVER_URI, "/submit_r2", post_data).await?;

    Ok(Json(SubmitRound2KeyResponse {}))
}

#[post("/get_dec_share", format = "json", data = "<request>")]
async fn get_dec_share(
    state: &State<SharedState>,
    request: Json<GetDecShareRequest>,
) -> Json<GetDecShareResponse> {
    let app_state = state.lock().await;

    let dec_share = app_state.user.decrypt_share(&request.ct);

    Json(GetDecShareResponse { dec_share })
}

async fn get_dec_shares(
    ct: &PhantomPackedCt,
) -> Result<[PhantomPackedCtDecShare; 3], Custom<String>> {
    let body = &GetDecShareRequest { ct: ct.clone() };
    OTHER_PLAYER_URIS
        .iter()
        .map(move |uri| async move {
            let client = reqwest::Client::new();
            let response = client
                .post(format!("{uri}/get_dec_share"))
                .json(body)
                .send()
                .await
                .map_err(internal_server_error)?;
            if response.status().is_success() {
                let body: GetDecShareResponse =
                    response.json().await.map_err(internal_server_error)?;
                Ok(body.dec_share)
            } else {
                let status = response.status();
                let body = response.text().await.map_err(internal_server_error)?;
                tracing::error!("Request failed with status: {status} body: {body}");
                Err(custom(status, body))
            }
        })
        .collect::<FuturesUnordered<_>>()
        .try_collect::<Vec<_>>()
        .await
        .map(|dec_shares| dec_shares.try_into().unwrap())
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
    let shared_state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState::new(*PLAYER_ID)));

    // Create a custom configuration
    let config = Config {
        port: *PORT,
        address: std::net::IpAddr::V4("0.0.0.0".parse().unwrap()),
        ..Config::default()
    };

    let _ = &*OTHER_PLAYER_URIS;

    rocket::custom(config)
        .manage(shared_state.clone())
        .mount(
            "/",
            routes![
                reset_game,
                mock_move,
                queue_move,
                mock_get_cells,
                get_cells,
                get_five_cells,
                get_cross_cells,
                get_vertical_cells,
                get_horizontal_cells,
                mock_get_player,
                get_player,
                get_id,
                set_id,
                get_pk,
                submit_r1,
                submit_r2,
                get_dec_share
            ],
        )
        .attach(make_cors())
        .launch()
        .await
        .map(|_rocket| ()) // Convert `Result<Rocket<Ignite>, rocket::Error>` to `Result<(), rocket::Error>`
}
