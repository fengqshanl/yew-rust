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
    pub id: String,
    pub name: String,
    pub self_money: f32,
    pub sale_money: f32,
    pub number: f32,
}