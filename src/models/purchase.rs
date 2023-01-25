use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::error::Error;
use uuid::Uuid;
use tokio_postgres::Row;
use postgres_types::{self, ToSql, FromSql};
use time;

#[derive(Deserialize, PostgresMapper, Serialize, Debug, ToSql, FromSql, Clone)]
#[pg_mapper(table = "purchase")]
pub struct Purchase {
    pub per_id: String,
    pub kind: i32,
    pub money: f32,
    pub in_time: String,
}

impl Purchase {
    pub fn from_row_ref(row: &Row) -> Result<Self, Error>{
        println!("models - purchase: {:?}", row);
        Ok(Purchase{
            per_id: row.try_get::<&str, Uuid>("per_id")?.hyphenated().to_string(),
            money: row.try_get::<&str, f32>("money")?,
            in_time: row.try_get::<&str, time::PrimitiveDateTime>("in_time")?.to_string(),
            kind: row.try_get::<&str, i32>("kind")?
        })
    }
}