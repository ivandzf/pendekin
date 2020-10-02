use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;
use std::io::Cursor;

#[derive(Debug)]
pub struct BaseResponse {
    status: Status,
    message: &'static str,
    data: JsonValue,
}

impl BaseResponse {
    pub fn data(mut self, data: JsonValue) -> BaseResponse {
        self.data = data;
        self
    }
}

impl<'r> Responder<'r> for BaseResponse {
    fn respond_to(self, request: &Request) -> Result<Response<'r>, Status> {
        let body = self.data;
        Response::build()
            .status(self.status)
            .sized_body(Cursor::new(body.to_string()))
            .header(ContentType::JSON)
            .ok()
    }
}

pub fn ok() -> BaseResponse {
    BaseResponse {
        status: Status::Ok,
        message: "Success",
        data: json!(null),
    }
}

pub fn created() -> BaseResponse {
    BaseResponse {
        status: Status::Created,
        message: "Created",
        data: json!(null),
    }
}
