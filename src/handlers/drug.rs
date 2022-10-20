use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

use crate::{db::drug, errors::errors::MyError, models::drug::Drug};

pub async fn add_drug(
    drug: web::Json<Drug>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let drug_info: Drug = drug.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_drug = drug::add_drug(&client, drug_info).await?;

    Ok(HttpResponse::Ok().json(new_drug))
}

pub async fn get_drug(
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_drug = drug::get_drug(&client).await?;
    Ok(HttpResponse::Ok().json(new_drug))
}