use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;
use crate::front::sale::FRSale;
use std::time::{SystemTime};
use crate::{errors::errors::MyError, models::sale::Sale};

pub async fn add_sale(client: &Client, sale_info: FRSale) -> Result<Sale, MyError> {
    let _stmt = include_str!("../../sql/sale/add_sale.sql");
    let _stmt = _stmt.replace("$table_fields", &Sale::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();
    println!("_smt: {:?}",_stmt);
    client
        .query(
            &stmt,
            &[
                &Uuid::parse_str(&sale_info.drug_id).expect("msg"),
                &sale_info.name,
                &sale_info.number,
                &sale_info.money,
                &sale_info.sale,
                &SystemTime::now(),
            ],
        )
        .await?
        .iter()
        .map(|row| {
            Sale::from_row_ref(row).unwrap()
        })
        .collect::<Vec<Sale>>()
        .pop()
        .ok_or(MyError::NotFound)
}

pub async fn get_sale(client: &Client) -> Result<Vec<Sale>, MyError> {
    let _stmt = include_str!("../../sql/sale/get_sale.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    Ok(client
        .query(
            &stmt, &[],
        )
        .await?
        .iter()
        .map(|row| {
            Sale::from_row_ref(row).unwrap()
        })
        .collect::<Vec<Sale>>())
}