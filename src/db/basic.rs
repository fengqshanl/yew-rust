use deadpool_postgres::Client;
use crate::errors::errors::MyError;
use crate::models::basic::SystemDict;

pub async fn get_dict(client: &Client, dict_type: String) -> Result<Vec<SystemDict>, MyError> {
    let _stmt = include_str!("../../sql/util/get_dict.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    Ok(client
        .query(&stmt, &[&dict_type])
        .await?
        .iter()
        .map(|row| {
            SystemDict::row_2_dict(row).unwrap()
        })
        .collect::<Vec<SystemDict>>())
}
