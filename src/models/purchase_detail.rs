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
    pub code: String,
    pub sale_money: f32,
    pub name: String,
    pub number: f32,
    pub self_money: f32,
    pub manu_address: String,
    pub spec: String
}

impl PurchaseDetail {
    pub fn from_row_ref(row: &Row) -> Result<Self, Error>{
        Ok(PurchaseDetail{
            purchase: row.try_get::<&str, Uuid>("purchase")?.hyphenated().to_string(),
            code: row.try_get::<&str, String>("code")?,
            sale_money: row.try_get::<&str, f32>("sale_money")?,
            self_money: row.try_get::<&str, f32>("self_money")?,
            number: row.try_get::<&str, f32>("number")?,
            name: row.try_get::<&str, String>("name")?,
            manu_address: row.try_get::<&str, String>("manu_address")?,
            spec: row.try_get::<&str, String>("spec")?,
        })
    }
}