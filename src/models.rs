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
    // pub drug_number: String,
    // pub ingredient: String,
    // pub character: String,
    // pub major_function: String,
    // pub specification: String,
    pub usage_dosage: String,
    // pub adverse_reaction: String,
    // pub taboo: String,
    pub matters_need_attention: String,
    // pub store_up: String,
    // pub expiry_date: i64,
    // pub produced_time: String,
    // pub approval_number: String,
    pub a_b_classify: String,
    // pub manufacturing_enterprise: String,
}

impl Drug {
    pub fn from_row_ref(row: &Row) -> Result<Self, Error>{
        println!("my from_row_ref");
        Ok(Drug{
            name: row.try_get::<&str, String>("name")?.to_string(),
            drug_id: row.try_get::<&str, Uuid>("drug_id")?.hyphenated().to_string(),
            class_id: row.try_get::<&str, Uuid>("class_id")?.hyphenated().to_string(),
            // drug_number: row.try_get::<&str, String>("drug_number")?.to_string(),
            // ingredient: row.try_get::<&str, String>("ingredient")?.to_string(),
            // character: row.try_get::<&str, String>("character")?.to_string(),
            // major_function: row.try_get::<&str, String>("major_function")?.to_string(),
            // specification: row.try_get::<&str, String>("specification")?.to_string(),
            usage_dosage: row.try_get::<&str, String>("usage_dosage")?.to_string(),
            // adverse_reaction: row.try_get::<&str, String>("adverse_reaction")?.to_string(),
            // taboo: row.try_get::<&str, String>("taboo")?.to_string(),
            matters_need_attention: row.try_get::<&str, String>("matters_need_attention")?.to_string(),
            // store_up: row.try_get::<&str, String>("store_up")?.to_string(),
            // expiry_date: row.try_get::<&str, i64>("expiry_date")?,
            // produced_time:  row.try_get::<&str, String>("produced_time")?.to_string(),
            // approval_number: row.try_get::<&str, String>("approval_number")?.to_string(),
            a_b_classify: row.try_get::<&str, String>("a_b_classify")?.to_string(),
            // manufacturing_enterprise: row.try_get::<&str, String>("manufacturing_enterprise")?.to_string()
        })
    }
}