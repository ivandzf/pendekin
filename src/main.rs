use actix_web::{web, App, HttpServer};
use std::io::Result;

pub mod api;
pub mod models;
pub mod services;

struct AppState {
    store: store_factory::PluginFactory,
}

#[actix_web::main]
async fn main() -> Result<()> {
    // init store
    let store = store_factory::PluginFactory::new("map".to_string());
    // init app_data to inject the routes
    let data = web::Data::new(AppState { store });
    // http server
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(api::shortener::index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
