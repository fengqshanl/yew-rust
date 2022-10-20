use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::{errors::errors::MyError, models::sale::Sale};

pub async fn add_sale(client: &Client, sale_info: Sale) -> Result<Sale, MyError> {
    let _stmt = include_str!("../../sql/sale/add_sale.sql");
    let _stmt = _stmt.replace("$table_fields", &Sale::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &sale_info.sale_id,
                &sale_info.drug_id,
                &sale_info.drug_name,
                &sale_info.sale_number,
                &sale_info.money,
                &sale_info.total,
                &sale_info.sale_time,
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