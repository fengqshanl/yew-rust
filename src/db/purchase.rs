use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::{errors::errors::MyError, models::purchase::Purchase, front::purchase::FRPurchase};

pub async fn add_purchase(client: &Client, purchase_info: FRPurchase) -> Result<Purchase, MyError> {
    let _stmt = include_str!("../../sql/purchase/add_purchase.sql");
    let _stmt = _stmt.replace("$table_fields", &Purchase::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &purchase_info.per_id,
                &purchase_info.money,
                &purchase_info.kind,
                &purchase_info.in_time,
            ],
        )
        .await?
        .iter()
        .map(|row| {
            Purchase::from_row_ref(row).unwrap()
        })
        .collect::<Vec<Purchase>>()
        .pop()
        .ok_or(MyError::NotFound)
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