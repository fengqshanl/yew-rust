use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::error::Error;
use uuid::Uuid;
use tokio_postgres::Row;
use postgres_types::{self, ToSql, FromSql};
use time;

#[derive(Deserialize, PostgresMapper, Serialize, Debug, ToSql, FromSql)]
#[pg_mapper(table = "purchase_detail0")]
pub struct FrontSalePurchaseDetail {
    pub purchase: String,
    pub drug_id: String,
    pub code: String,
    pub sale_money: f32,
    pub name: String,
    pub manu_address: String,
    pub spec: String
}

impl FrontSalePurchaseDetail {
    pub fn from_row_ref(row: &Row) -> Result<Self, Error>{
        Ok(FrontSalePurchaseDetail{
            purchase: row.try_get::<&str, Uuid>("purchase")?.hyphenated().to_string(),
            drug_id: row.try_get::<&str, Uuid>("drug_id")?.hyphenated().to_string(),
            code: row.try_get::<&str, String>("code")?,
            sale_money: row.try_get::<&str, f32>("sale_money")?,
            name: row.try_get::<&str, String>("name")?,
            manu_address: row.try_get::<&str, String>("manu_address")?,
            spec: row.try_get::<&str, String>("spec")?,
        })
    }
}