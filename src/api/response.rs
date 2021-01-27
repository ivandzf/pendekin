use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseResponse<T> {
    status: u16,
    message: &'static str,
    data: T,
}

pub fn ok<T>(data: T) -> BaseResponse<T> {
    BaseResponse {
        status: StatusCode::OK.as_u16(),
        message: "Success",
        data,
    }
}

pub fn created<T>(data: T) -> BaseResponse<T> {
    BaseResponse {
        status: StatusCode::CREATED.as_u16(),
        message: "Created",
        data,
    }
}
