use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::front::purchase::FRPurchaseType;
use uuid::Uuid;
use crate::{errors::errors::MyError, models::{purchase::Purchase, purchase_detail::PurchaseDetail}};

pub async fn add_detail(client: &Client, purchase_detail: FRPurchaseType, purchase_id: String) -> Result<(), MyError> {
    let _stmt = include_str!("../../sql/purchase_detail/add_detail.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    println!("_stmt: => {:?}", _stmt);
    client
        .query(
            &stmt,
            &[
                &Uuid::parse_str(&purchase_id).expect("err"),
                &purchase_detail.code,
                &purchase_detail.number.parse::<f32>().expect("msg"),
                &purchase_detail.name,
                &purchase_detail.sale_money.parse::<f32>().expect("msg"),
                &purchase_detail.self_money.parse::<f32>().expect("msg"),
                &purchase_detail.spec,
                &purchase_detail.manu_address
            ],
        )
        .await.expect("msg");
        Ok(())
}