use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::front::purchase::FRPurchaseType;
use uuid::Uuid;
use crate::{errors::errors::MyError, models::{purchase::Purchase, purchase_detail::PurchaseDetail}};

pub async fn add_detail(client: &Client, purchase_detail: FRPurchaseType, purchase_id: String) -> Result<Purchase, MyError> {
    let _stmt = include_str!("../../sql/purchase_detail/add_detail.sql");
    let _stmt = _stmt.replace("$table_fields", &PurchaseDetail::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();
    client
        .query(
            &stmt,
            &[
                &Uuid::parse_str(&purchase_id).expect("err"),
                &Uuid::parse_str(&purchase_detail.id).expect("msg"),
                &purchase_detail.number,
                &purchase_detail.name,
                &purchase_detail.sale_money,
                &purchase_detail.self_money,
            ],
        )
        .await?.iter()
        .map(|row| {
            Purchase::from_row_ref(row).unwrap()
        })
        .collect::<Vec<Purchase>>()
        .pop()
        .ok_or(MyError::NotFound)
}