use deadpool_postgres::Client;
use uuid::Uuid;
use std::time::{SystemTime};
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::models::sale::{sale::Sale ,frontend::FrontSale};
use crate::{errors::errors::MyError};

pub async fn add_sale(client: &Client, sale_info: FrontSale) -> Result<Vec<Sale>, MyError> {
    let _stmt = include_str!("../../sql/sale/add_sale.sql");
    let _stmt = _stmt.replace("$table_fields", &Sale::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();
    Ok(client
        .query(
            &stmt,
            &[
                &Uuid::parse_str(&sale_info.drug_id).expect("msg"),
                &sale_info.name,
                &sale_info.number,
                &sale_info.sale_money,
                &sale_info.total,
                &SystemTime::now(),
                &sale_info.code,
                &sale_info.spec,
                &sale_info.manu_name,
            ],
        )
        .await?
        .iter()
        .map(|row| {
            Sale::from_row_ref(row).unwrap()
        })
        .collect::<Vec<Sale>>())
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