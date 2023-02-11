use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

use crate::{db::drug, errors::errors::MyError, models::drug::{local::DrugId}};

pub async fn get_drug(
    drug_id: web::Query<DrugId>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_drug = drug::get_drug(&client, drug_id.id.clone()).await?;
    Ok(HttpResponse::Ok().json(new_drug))
}