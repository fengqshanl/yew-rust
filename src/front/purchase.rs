use serde::{Deserialize, Serialize};
use postgres_types::{self, ToSql, FromSql};

#[derive(Deserialize, Serialize, Debug, ToSql, FromSql, Clone)]
pub struct FRPurchase {
    pub per_id: String,
    pub kind: i32,
    pub money: f32,
    pub in_time: String,
    pub detail: Vec<FRPurchaseType>
}

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize, ToSql, FromSql)]
pub struct FRPurchaseType {
    pub drug_id: String,
    pub code: String,
    pub name: String,
    pub self_money: String,
    pub sale_money: String,
    pub number: String,
    pub spec: String,
    pub manu_address: String,
}