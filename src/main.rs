mod config;
mod db;
mod errors;
mod handlers;
mod models;

use ::config::Config;
use actix_web::{App, web, HttpServer, http::header};
use dotenv::dotenv;
use actix_cors::Cors;
use handlers::handlers::{add_drug, get_drug};
use tokio_postgres::NoTls;
use crate::config::config::ExampleConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: ExampleConfig = config_.try_deserialize().unwrap();

    println!("config: {:?}, default: {:?}", config, ::config::Environment::default());

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Cors::permissive())
            .service(web::resource("/drug")
                .route(web::post().to(add_drug))
                .route(web::get().to(get_drug))
            )
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}