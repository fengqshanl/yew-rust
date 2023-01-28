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
    pub drug_name: String,
    pub sale_number: f32,
    pub money: f32,
    pub total: f32,
    pub sale_time: String,
}

impl Sale {
    pub fn from_row_ref(row: &Row) -> Result<Self, Error>{
        Ok(Sale{
            sale_id: row.try_get::<&str, Uuid>("sale_id")?.hyphenated().to_string(),
            drug_id: row.try_get::<&str, Uuid>("drug_id")?.hyphenated().to_string(),
            drug_name: row.try_get::<&str, String>("drug_name")?.to_string(),
            sale_number: row.try_get::<&str, f32>("sale_number")?,
            money: row.try_get::<&str, f32>("money")?,
            total: row.try_get::<&str, f32>("total")?,
            sale_time: row.try_get::<&str, time::PrimitiveDateTime>("sale_time")?.to_string(),
        })
    }
}