use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::error::Error;
use uuid::Uuid;
use tokio_postgres::Row;
use postgres_types::{self, ToSql, FromSql};

#[derive(Deserialize, PostgresMapper, Serialize, Debug, ToSql, FromSql)]
#[pg_mapper(table = "classification")]
pub struct Classification {
    pub class_id: String,
    pub classf_name: String,
}

impl Classification {
    pub fn from_row_ref(row: &Row) -> Result<Self, Error>{
        Ok(Classification{
            class_id: row.try_get::<&str, Uuid>("class_id")?.hyphenated().to_string(),
            classf_name: row.try_get::<&str, String>("classf_name")?.to_string(),
        })
    }
}