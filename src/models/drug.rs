use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::error::Error;
use uuid::Uuid;
use tokio_postgres::Row;
use postgres_types::{self, ToSql, FromSql};

#[derive(Deserialize, PostgresMapper, Serialize, Debug, ToSql, FromSql)]
#[pg_mapper(table = "drug")]
pub struct Drug {
    pub name: String,
    pub drug_id: String,
    pub class_id: String,
    pub usage_dosage: String,
    pub matters_need_attention: String,
    pub a_b_classify: String,
}

impl Drug {
    pub fn from_row_ref(row: &Row) -> Result<Self, Error>{
        println!("my from_row_ref");
        Ok(Drug{
            name: row.try_get::<&str, String>("name")?.to_string(),
            drug_id: row.try_get::<&str, Uuid>("drug_id")?.hyphenated().to_string(),
            class_id: row.try_get::<&str, Uuid>("class_id")?.hyphenated().to_string(),
            usage_dosage: row.try_get::<&str, String>("usage_dosage")?.to_string(),
            matters_need_attention: row.try_get::<&str, String>("matters_need_attention")?.to_string(),
            a_b_classify: row.try_get::<&str, String>("a_b_classify")?.to_string(),
        })
    }
}