#[macro_use] 
extern crate rocket;

use dotenv::dotenv;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

mod db;
mod errors;
mod fairings;
mod models;
mod request_guards;
mod controllers;


#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
    .attach(db::init())
    .mount(
        "/api-docs",
        make_swagger_ui(&SwaggerUIConfig {
            url: "../api/openapi.json".to_owned(),
            ..Default::default()
        }),
    )
    .mount(
        "/api/", 
        openapi_get_routes![
            controllers::index,
            controllers::customer::post_customer,
            controllers::customer::get_customers,
            controllers::customer::get_customer_by_id,
            controllers::customer::patch_customer_by_id,
            controllers::customer::delete_customer_by_id,
        ]
    )
}