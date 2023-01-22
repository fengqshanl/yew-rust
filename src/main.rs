mod config;
mod db;
mod errors;
mod handlers;
mod models;
mod front;

use crate::config::config::ExampleConfig;
use ::config::Config;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use handlers::{
    drug::{add_drug, get_drug, get_all_drug, search_drug_name},
    purchase::{add_purchase, get_purchase},
    sale::{add_sale, get_sale},
};
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: ExampleConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Cors::permissive())
            .service(
                web::resource("/drug")
                    .route(web::post().to(add_drug))
                    .route(web::get().to(get_drug)),
            )
            .service(
                web::resource("/all")
                    .route(web::get().to(get_all_drug))
            )
            .service(
                web::resource("/search_drug_name")
                    .route(web::put().to(search_drug_name))   
            )
            .service(
                web::resource("/sale")
                    .route(web::post().to(add_sale))
                    .route(web::get().to(get_sale)),
            )
            .service(
                web::resource("/purchase")
                    .route(web::post().to(add_purchase))
                    .route(web::get().to(get_purchase)),
            )
    })
    .bind(config.server_addr.clone())?
    .run();

    server.await
}
