use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

use crate::{
    db::{purchase, purchase_detail}, 
    errors::errors::MyError, 
    front::purchase::FRPurchase
};

pub async fn add_purchase(
    purchase: web::Json<FRPurchase>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let purchase_info: FRPurchase = purchase.into_inner();
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let id = purchase_info.clone().per_id;
    let new_purchase = purchase::add_purchase(&client, purchase_info.clone()).await?;
    println!("new_purchase:{:?}", new_purchase);
    for row in purchase_info.detail {
        purchase_detail::add_detail(&client, row, id.clone()).await?;
    }
    
    Ok(HttpResponse::Ok().json(new_purchase))
}

pub async fn get_purchase(
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_purchase = purchase::get_purchase(&client).await?;
    Ok(HttpResponse::Ok().json(new_purchase))
}