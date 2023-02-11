use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::error::Error;
use uuid::Uuid;
use tokio_postgres::Row;
use postgres_types::{self, ToSql, FromSql};

#[derive(Deserialize, PostgresMapper, Serialize, Debug, ToSql, FromSql)]
#[pg_mapper(table = "sale")]
pub struct Sale {
    pub sale_id: String,
    pub drug_id: String,
    pub name: String,
    pub number: f32,
    pub money: f32,
    pub code: String,
    pub total: f32,
    pub sale_time: String,
    pub spec: String,
    pub manu_address: String,
}

impl Sale {
    pub fn from_row_ref(row: &Row) -> Result<Self, Error>{
        Ok(Sale{
            sale_id: row.try_get::<&str, Uuid>("sale_id")?.hyphenated().to_string(),
            drug_id: row.try_get::<&str, Uuid>("drug_id")?.hyphenated().to_string(),
            name: row.try_get::<&str, String>("name")?.to_string(),
            spec: row.try_get::<&str, String>("spec")?.to_string(),
            manu_address: row.try_get::<&str, String>("manu_address")?.to_string(),
            code: row.try_get::<&str, String>("code")?.to_string(),
            number: row.try_get::<&str, f32>("number")?,
            money: row.try_get::<&str, f32>("money")?,
            total: row.try_get::<&str, f32>("total")?,
            sale_time: row.try_get::<&str, time::PrimitiveDateTime>("sale_time")?.to_string(),
        })
    }
}