use serde::{Deserialize, Serialize};
use tokio_postgres::error::Error;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct FrontDrugDetail {
    pub drug_id: String,
    pub name: String,
    pub usage_dosage: String,
    pub matters_need_attention: String,
}

impl FrontDrugDetail {
    pub fn from_row_ref(row: &Row) -> Result<Self, Error>{
        Ok(FrontDrugDetail{
            drug_id: row.try_get::<&str, Uuid>("drug_id")?.hyphenated().to_string(),
            name: row.try_get::<&str, String>("name")?.to_string(),
            usage_dosage: row.try_get::<&str, String>("usage_dosage")?.to_string(),
            matters_need_attention: row.try_get::<&str, String>("matters_need_attention")?.to_string()
        })
    }
}
