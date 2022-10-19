use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::error::Error;
use uuid::Uuid;
use tokio_postgres::Row;
use postgres_types::{self, ToSql, FromSql};

#[derive(Deserialize, PostgresMapper, Serialize, Debug, ToSql, FromSql)]
#[pg_mapper(table = "broth_song")]
pub struct Broth {
    pub broth_id: String,
    pub broth_name: String,
    pub broth_content: String,
}


impl Broth {
    pub fn from_row_ref(row: &Row) -> Result<Self, Error>{
        Ok(Broth{
            broth_id: row.try_get::<&str, Uuid>("broth_id")?.hyphenated().to_string(),
            broth_name: row.try_get::<&str, String>("broth_name")?.to_string(),
            broth_content: row.try_get::<&str, String>("broth_content")?.to_string(),
        })
    }
}