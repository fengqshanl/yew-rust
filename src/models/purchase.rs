use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::error::Error;
use uuid::Uuid;
use tokio_postgres::Row;
use postgres_types::{self, ToSql, FromSql};

#[derive(Deserialize, PostgresMapper, Serialize, Debug, ToSql, FromSql)]
#[pg_mapper(table = "purchase")]
pub struct Purchase {
    pub per_id: String,
    pub stock: String,
    pub money: String,
    pub drug_id: String,
}

impl Purchase {
    pub fn from_row_ref(row: &Row) -> Result<Self, Error>{
        println!("my from_row_ref");
        Ok(Purchase{
            per_id: row.try_get::<&str, Uuid>("per_id")?.hyphenated().to_string(),
            stock: row.try_get::<&str, String>("stock")?.to_string(),
            money: row.try_get::<&str, String>("money")?.to_string(),
            drug_id: row.try_get::<&str, Uuid>("drug_id")?.hyphenated().to_string(),
        })
    }
}