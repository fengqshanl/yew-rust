use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize, Debug)]
#[pg_mapper(table = "drug")]
pub struct Drug {
    pub name: String,
    pub drug_id: String,
    pub drug_number: String,
    pub ingredient: String,
    pub character: String,
    pub major_function: String,
    pub specification: String,
    pub usage_dosage: String,
    pub adverse_reaction: String,
    pub taboo: String,
    pub matters_need_attention: String,
    pub store_up: String,
    pub expiry_date: i64,
    pub produced_time: String,
    pub approval_number: String,
    pub manufacturing_enterprise: String,
}