use serde::{Deserialize};

#[derive(Debug, Deserialize, Clone)]
pub struct DrugId {
   pub id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DrugDetailId {
   pub id: String,
}
