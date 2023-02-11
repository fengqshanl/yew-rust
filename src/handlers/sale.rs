use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};
use crate::{db::sale, errors::errors::MyError};
use crate::models::sale::frontend::FrontSale;

pub async fn add_sale(
    sale: web::Json<FrontSale>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_sale = sale::add_sale(&client, sale.into_inner()).await?;
    Ok(HttpResponse::Ok().json(new_sale))
}

pub async fn get_sale(
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_sale = sale::get_sale(&client).await?;
    Ok(HttpResponse::Ok().json(new_sale))
}