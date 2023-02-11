use deadpool_postgres::Client;
use crate::{front::purchase::FRPurchaseType, models::purchase_detail::{purchase_detail::{PurchaseDetail}, frontend::FrontSalePurchaseDetail}};
use uuid::Uuid;
use crate::{errors::errors::MyError};

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
                &purchase_detail.manu_address,
                &Uuid::parse_str(&purchase_detail.drug_id).expect("err"),
            ],
        )
        .await.expect("msg");
        Ok(())
}

pub async fn get_purchase_detail(client: &Client, id: String) -> Result<Vec<PurchaseDetail>, MyError> {
    let _stmt = include_str!("../../sql/purchase_detail/get_detail.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    Ok(client
        .query(
            &stmt, &[&Uuid::parse_str(&id).expect("err")],
        )
        .await?
        .iter()
        .map(|row| {
            PurchaseDetail::from_row_ref(row).unwrap()
        })
        .collect::<Vec<PurchaseDetail>>())
}

pub async fn get_sale_purchase_detail(client: &Client, id: String) -> Result<FrontSalePurchaseDetail, MyError> {
    let _stmt = include_str!("../../sql/purchase_detail/get_sale_detail.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    client
        .query(
            &stmt, &[&id],
        )
        .await?
        .iter()
        .map(|row| {
            FrontSalePurchaseDetail::from_row_ref(row).unwrap()
        })
        .collect::<Vec<FrontSalePurchaseDetail>>().pop().ok_or(MyError::NotFound)
}