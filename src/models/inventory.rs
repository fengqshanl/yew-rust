use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::error::Error;
use uuid::Uuid;
use tokio_postgres::Row;
use postgres_types::{self, ToSql, FromSql};

#[derive(Deserialize, PostgresMapper, Serialize, Debug, ToSql, FromSql)]
#[pg_mapper(table = "inventory")]
pub struct Inventory {
    pub inv_id: String,
    pub drug_id: String,
    pub stock: String,
    pub money: String,
    pub warning: String,
}

impl Inventory {
    pub fn from_row_ref(row: &Row) -> Result<Self, Error>{
        Ok(Inventory{
            inv_id: row.try_get::<&str, Uuid>("inv_id")?.hyphenated().to_string(),
            drug_id: row.try_get::<&str, Uuid>("drug_id")?.hyphenated().to_string(),
            stock: row.try_get::<&str, String>("stock")?.to_string(),
            money: row.try_get::<&str, String>("money")?.to_string(),
            warning: row.try_get::<&str, String>("warning")?.to_string(),
        })
    }
}