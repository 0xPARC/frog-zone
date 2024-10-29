use rocket::{http::Status, response::status::Custom};

pub mod client;
pub mod mock_zone;
pub mod temp;
pub mod worker;
pub mod zone;

pub fn bad_request(err: impl ToString) -> Custom<String> {
    custom(Status::BadRequest, err)
}

pub fn internal_server_error(err: impl ToString) -> Custom<String> {
    custom(Status::from_code(500).unwrap(), err)
}

pub fn custom(status: Status, err: impl ToString) -> Custom<String> {
    Custom(status, err.to_string())
}
