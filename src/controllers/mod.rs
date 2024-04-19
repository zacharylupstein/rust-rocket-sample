use rocket::serde::json::Json;
use rocket_okapi::openapi;

use crate::models::response::IndexResponse;

pub mod customer;

/// This is a description. <br />You can do simple html <br /> like <b>this<b/>
#[openapi(tag = "Hello World")]
#[get("/")]
pub fn index() -> Json<IndexResponse> {
    Json(IndexResponse {
        message: "Hello Zach!".to_string(),
        testing: "chicken".to_string(),
        this_worked: true,
    })
}