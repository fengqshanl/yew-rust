use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::{errors::errors::MyError, models::{purchase::Purchase, purchase_detail::PurchaseDetail}};

pub async fn add_detail(client: &Client, purchase_detail: FRPurchaseType, purchase_id: String) -> Result<Purchase, MyError> {
    let _stmt = include_str!("../../sql/purchase_detail/add_detail.sql");
    let _stmt = _stmt.replace("$table_fields", &PurchaseDetail::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &purchase_id,
                &purchase_detail.id,
                &purchase_detail.number,
                &purchase_detail.name,
                &purchase_detail.sale_money,
                &purchase_detail.self_money,
            ],
        )
        .await?
}

    // pub id: String,
    // pub name: String,
    // pub self_money: f32,
    // pub sale_money: f32,
    // pub number: f32,