use rocket::{
    http::Status,
    serde::{json::Json, Serialize},
};

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    status: String,
    title: String,
    detail: String,
}

pub fn error_response(status: Status, title: &str, detail: &str) -> (Status, Json<ErrorResponse>) {
    (
        status,
        Json(ErrorResponse {
            status: status.to_string(),
            title: title.into(),
            detail: detail.into(),
        }),
    )
}
