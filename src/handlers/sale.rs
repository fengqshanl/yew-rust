use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};
use crate::front::sale::FRSale;
use crate::{db::sale, errors::errors::MyError};

pub async fn add_sale(
    sale: web::Json<Vec<FRSale>>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let sale_info: Vec<FRSale> = sale.into_inner();
    println!("sale info{:?}", sale_info);
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    println!("client:");

    for row in sale_info {
        let new_sale = sale::add_sale(&client, row).await?;
        println!("new sale: {:?}", new_sale);
    }

    Ok(HttpResponse::Ok().json("ok"))
}

pub async fn get_sale(
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_sale = sale::get_sale(&client).await?;
    Ok(HttpResponse::Ok().json(new_sale))
}