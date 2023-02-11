mod config;
mod db;
mod errors;
mod handlers;
mod models;
mod client;
mod front;

use crate::config::config::ExampleConfig;
use ::config::Config;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, middleware};
use dotenv::dotenv;
use handlers::{
    drug::{get_drug},
    purchase::{add_purchase, get_purchase, get_purchase_detail, get_sale_purchase_detail},
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
            .wrap(middleware::Logger::default())
            .wrap(Cors::permissive())
            .service(
                web::resource("/drug")
                    .route(web::get().to(get_drug)),
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
            .service(
                web::resource("/purchase_detail")
                    .route(web::get().to(get_purchase_detail)),
            )
            .service(
                web::resource("/purchase_sale_detail")
                    .route(web::get().to(get_sale_purchase_detail)),
            )
    })
    .bind(config.server_addr.clone())?
    .run();

    server.await
}
