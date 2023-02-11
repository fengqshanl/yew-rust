use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;
use std::time::{SystemTime};
use crate::{errors::errors::MyError, front::purchase::FRPurchase, models::purchase::purchase::Purchase};

pub async fn add_purchase(client: &Client, purchase_info: FRPurchase) -> Result<(), MyError> {
    let _stmt = include_str!("../../sql/purchase/add_purchase.sql");
    let _stmt = _stmt.replace("$table_fields", &Purchase::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();
    client
        .query(
            &stmt,
            &[
                &Uuid::parse_str(&purchase_info.per_id).expect("msg"),
                &purchase_info.money,
                &purchase_info.kind,
                &SystemTime::now(),
            ],
        )
        .await.expect("msg");
        Ok(())
}

pub async fn get_purchase(client: &Client) -> Result<Vec<Purchase>, MyError> {
    let _stmt = include_str!("../../sql/purchase/get_purchase.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    Ok(client
        .query(
            &stmt, &[],
        )
        .await?
        .iter()
        .map(|row| {
            Purchase::from_row_ref(row).unwrap()
        })
        .collect::<Vec<Purchase>>())
}

