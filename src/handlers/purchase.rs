use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

use crate::{db::purchase, errors::errors::MyError, models::purchase::Purchase};

pub async fn add_purchase(
    purchase: web::Json<Purchase>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let purchase_info: Purchase = purchase.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_purchase = purchase::add_purchase(&client, purchase_info).await?;

    Ok(HttpResponse::Ok().json(new_purchase))
}

pub async fn get_purchase(
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_purchase = purchase::get_purchase(&client).await?;
    Ok(HttpResponse::Ok().json(new_purchase))
}