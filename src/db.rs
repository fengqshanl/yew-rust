use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{errors::MyError, models::Drug};

pub async fn add_drug(client: &Client, drug_info: Drug) -> Result<Drug, MyError> {
    let _stmt = include_str!("../sql/add_drug.sql");
    let _stmt = _stmt.replace("$table_fields", &Drug::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &drug_info.drug_id,
                &drug_info.class_id,
                &drug_info.name,
                // &drug_info.adverse_reaction,
                // &drug_info.character,
                // &drug_info.approval_number,
                // &drug_info.ingredient,
                // &drug_info.major_function,
                // &drug_info.manufacturing_enterprise,
                &drug_info.matters_need_attention,
                // &drug_info.produced_time,
                // &drug_info.specification,
                // &drug_info.expiry_date,
                // &drug_info.store_up,
                // &drug_info.taboo,
                &drug_info.usage_dosage,
                &drug_info.a_b_classify,
            ],
        )
        .await?
        .iter()
        .map(|row| {
            println!("row");
            Drug::from_row_ref(row).unwrap()
        })
        .collect::<Vec<Drug>>()
        .pop()
        .ok_or(MyError::NotFound)
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
        .map(|row| {
            println!("row: {:?}", row);
            Drug::from_row_ref(row).unwrap()
        })
        .collect::<Vec<Drug>>())
}