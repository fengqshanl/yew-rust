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
    pub sale_number: String,
    pub money: String,
    pub total: String,
    pub sale_time: String,
}

impl Sale {
    pub fn from_row_ref(row: &Row) -> Result<Self, Error>{
        Ok(Sale{
            sale_id: row.try_get::<&str, Uuid>("sale_id")?.hyphenated().to_string(),
            drug_id: row.try_get::<&str, Uuid>("drug_id")?.hyphenated().to_string(),
            drug_name: row.try_get::<&str, String>("drug_name")?.to_string(),
            sale_number: row.try_get::<&str, String>("sale_number")?.to_string(),
            money: row.try_get::<&str, String>("money")?.to_string(),
            total: row.try_get::<&str, String>("total")?.to_string(),
            sale_time: row.try_get::<&str, String>("sale_time")?.to_string(),
        })
    }
}