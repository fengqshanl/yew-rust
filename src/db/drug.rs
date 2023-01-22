use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::{errors::errors::MyError, models::drug::Drug};

pub async fn add_drug(client: &Client, drug_info: Drug) -> Result<Drug, MyError> {
    let _stmt = include_str!("../../sql/drug/add_drug.sql");
    let _stmt = _stmt.replace("$table_fields", &Drug::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &drug_info.drug_id,
                &drug_info.class_id,
                &drug_info.name,
                &drug_info.matters_need_attention,
                &drug_info.usage_dosage,
                &drug_info.a_b_classify,
            ],
        )
        .await?
        .iter()
        .map(|row| {
            Drug::from_row_ref(row).unwrap()
        })
        .collect::<Vec<Drug>>()
        .pop()
        .ok_or(MyError::NotFound)
}

pub async fn get_drug(client: &Client) -> Result<Vec<Drug>, MyError> {
    let _stmt = include_str!("../../sql/drug/get_drug.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    Ok(client
        .query(
            &stmt, &[],
        )
        .await?
        .iter()
        .map(|row| {
            Drug::from_row_ref(row).unwrap()
        })
        .collect::<Vec<Drug>>())
}

pub async fn get_all_drug(client: &Client) -> Result<Vec<Drug>, MyError> {
    let _stmt = include_str!("../../sql/drug/get_all_drug.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    Ok(client
        .query(
            &stmt, &[],
        )
        .await?
        .iter()
        .map(|row| {
            Drug::from_row_ref(row).unwrap()
        })
        .collect::<Vec<Drug>>())
}

pub async fn search_drug_name(client: &Client, name: &str) -> Result<Vec<Drug>, MyError> {
    let _stmt = include_str!("../../sql/drug/search_drug_name.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    println!("here");
    Ok(client
        .query(
            &stmt, &[&name],
        )
        .await?
        .iter()
        .map(|row| {
            Drug::from_row_ref(row).unwrap()
        })
        .collect::<Vec<Drug>>())
}
