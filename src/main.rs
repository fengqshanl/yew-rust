mod config;
mod db;
mod errors;
mod handlers;
mod models;
mod client;
mod front;
mod util;

use chrono::prelude::*;
use crate::config::config::ExampleConfig;
use ::config::Config;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, middleware};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use handlers::{
    drug::{get_drug, search_drug}, medicine::{search_medicine, create_medicine, modify_medicine, delete_medicine, quick_search},
    purchase::{add_purchase, get_purchase, get_purchase_detail, get_sale_purchase_detail},
    sale::{add_sale, get_sale, change_sale, delete_sale},
    basic::{get_dict}
};
use tokio_postgres::NoTls;
use crate::handlers::medicine::get_medicine_detail;

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
            .wrap(Logger::new("%a \n %{User-Agent}i \n request URL %U \n request started %t \n response status code %s \n Size of response in bytes %b \n Real IP %{r}a "))
            .wrap(Cors::permissive())
            .service(
                web::resource("/drug")
                    .route(web::get().to(get_drug))
            )
            .service(
                web::resource("/search_drug")
                    .route(web::get().to(search_drug)),
            )
            .service(
                web::resource("/medicine")
                    .route(web::get().to(search_medicine))
                    .route(web::post().to(create_medicine))
                    .route(web::put().to(modify_medicine))
                    .route(web::delete().to(delete_medicine))
            )
            .service(
                web::resource("/medicine_detail")
                    .route(web::get().to(get_medicine_detail))
            )
            .service(
                web::resource("/quickSearch")
                    .route(web::get().to(quick_search))
            )
            .service(
                web::resource("/sale")
                    .route(web::post().to(add_sale))
                    .route(web::put().to(change_sale))
                    .route(web::delete().to(delete_sale))
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
            .service(
                web::resource("/dict")
                    .route(web::get().to(get_dict))
            )
    })
    .bind(config.server_addr.clone())?
    .run();

    server.await
}
