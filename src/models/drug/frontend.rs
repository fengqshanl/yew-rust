use serde::{Deserialize, Serialize};
use tokio_postgres::error::Error;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct FrontDrugDetail {
    pub drug_id: String,
    pub code: String, // 条形码
    pub goods_name: String, // 商品名称
    pub spec: String, // 规格
    pub manu_name: String, // 厂商
    pub sale_money: String,
}

impl FrontDrugDetail {
    pub fn from_row_ref(row: &Row) -> Result<Self, Error>{
        Ok(FrontDrugDetail{
            drug_id: row.try_get::<&str, Uuid>("drug_id")?.hyphenated().to_string(),
            code: row.try_get::<&str, String>("code")?.to_string(),
            goods_name: row.try_get::<&str, String>("goods_name")?.to_string(),
            spec: row.try_get::<&str, String>("spec")?.to_string(),
            manu_name: row.try_get::<&str, String>("manu_name")?.to_string(),
            sale_money: row.try_get::<&str, String>("manu_name")?.to_string(),
        })
    }
}