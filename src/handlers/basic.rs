use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

use crate::{db::basic, errors::errors::MyError, models::basic::DictQuery};

pub async fn get_dict(
    dict_type: web::Query<DictQuery>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_drug = basic::get_dict(&client, dict_type.dict_type.clone()).await?;
    Ok(HttpResponse::Ok().json(new_drug))
}
