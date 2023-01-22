use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::error::Error;
use uuid::Uuid;
use tokio_postgres::Row;
use postgres_types::{self, ToSql, FromSql};
use time;

#[derive(Deserialize, PostgresMapper, Serialize, Debug, ToSql, FromSql)]
#[pg_mapper(table = "purchase_detail0")]
pub struct PurchaseDetail {
    pub purchase: String,
    pub drug_id: String,
    pub sale_money: f32,
    pub name: String,
    pub number: i32,
    pub self_money: f32,
}

impl PurchaseDetail {
    pub fn from_row_ref(row: &Row) -> Result<Self, Error>{
        Ok(PurchaseDetail{
            purchase: row.try_get::<&str, Uuid>("purchase")?.hyphenated().to_string(),
            drug_id: row.try_get::<&str, Uuid>("drug_id")?.hyphenated().to_string(),
            sale_money: row.try_get::<&str, f32>("sale_money").expect("purchase money error"),
            self_money: row.try_get::<&str, f32>("self_money").expect("purchase money error"),
            number: row.try_get::<&str, i32>("number").expect("purchase kind error"),
            name: row.try_get::<&str, String>("name").expect("purchase kind error"),
        })
    }
}