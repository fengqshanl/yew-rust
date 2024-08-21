use deadpool_postgres::Client;
use crate::errors::errors::MyError;

pub async fn validate_dict(client: &Client, dict_type: String, dict_value: String) -> Result<(), MyError> {
    let _stmt = include_str!("../../sql/util/validate_dict.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    return match client
        .query(
            &stmt, &[
                &dict_type,
                &dict_value,
            ],
        )
        .await {
        Ok(row) => {
            if row.len() == 0 {
                return Err(MyError::NotFound);
            }
            Ok(())
        },
        Err(_) => {
            Err(MyError::NotFound)
        }
    }
}
