use rocket::get;
use rocket_contrib::json;

use crate::api::response::{ok, BaseResponse};

#[get("/")]
pub fn index() -> BaseResponse {
    ok().data(json!(null))
}
