use oxide_auth::endpoint::ResponseStatus;
use reqwest::StatusCode;


pub fn response_status_code(response_status: ResponseStatus) -> StatusCode {
    match response_status {
        ResponseStatus::Ok => StatusCode::OK,
        ResponseStatus::Unauthorized => StatusCode::UNAUTHORIZED,
        ResponseStatus::BadRequest => StatusCode::BAD_REQUEST,
        ResponseStatus::Redirect => StatusCode::PERMANENT_REDIRECT,
    }
}
