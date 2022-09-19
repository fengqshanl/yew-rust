use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{errors::MyError, models::Drug};

pub async fn add_drug(client: &Client, drug_info: Drug) -> Result<Drug, MyError> {
    println!("insert options:{:?}", drug_info);
    let _stmt = include_str!("../sql/add_drug.sql");
    let _stmt = _stmt.replace("$table_fields", &Drug::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &drug_info.id,
                &drug_info.name,
                &drug_info.age,
            ],
        )
        .await?
        .iter()
        .map(|row| Drug::from_row_ref(row).unwrap())
        .collect::<Vec<Drug>>()
        .pop()
        .ok_or(MyError::NotFound) // more applicable for SELECTs
}

pub async fn get_drug(client: &Client) -> Result<Vec<Drug>, MyError> {
    println!("search options");
    let _stmt = include_str!("../sql/get_drug.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    Ok(client
        .query(
            &stmt, &[],
        )
        .await?
        .iter()
        .map(|row| Drug::from_row_ref(row).unwrap())
        .collect::<Vec<Drug>>())
}