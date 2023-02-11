use serde::{Deserialize, Serialize};
use postgres_types::{self, ToSql, FromSql};

#[derive(Deserialize, Serialize, Debug, ToSql, FromSql, Clone)]
pub struct FRSale {
    pub drug_id: String,
    pub code: String,
    pub money: f32,
    pub number: f32,
    pub total: f32,
    pub name: String,
}
