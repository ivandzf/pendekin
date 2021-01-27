use actix_web::{get, HttpResponse};

use crate::api::response::ok;

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().json(ok({}))
}
