use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::error::Error;
use uuid::Uuid;
use tokio_postgres::Row;
use postgres_types::{self, ToSql, FromSql};
use time;

#[derive(Deserialize, PostgresMapper, Serialize, Debug, ToSql, FromSql, Clone)]
#[pg_mapper(table = "medicine")]
pub struct Medicine {
    pub medicine_id: String,
    pub material_name: String,
    pub purchasing_price: f32,
    pub selling_price: f32,
    pub traceability_code: String,
    pub material_alias: String,
    pub manufacturer: String,
    pub storage_condition: String,
    pub status: String,
    pub purchasing_base: f32,
    pub create_by: String,
    pub create_time: String,
    pub update_by: String,
    pub update_time: String,
    pub delete_by: String,
    pub delete_time: String,
}

impl Medicine {
    pub fn row_2_search(row: &Row) -> Result<Self, Error>{
        Ok(Medicine{
            medicine_id: row.try_get::<&str, Uuid>("medicine_id")?.hyphenated().to_string(),
            material_name: row.try_get::<&str, String>("material_name")?.to_string(),
            material_alias: row.try_get::<&str, String>("material_alias")?.to_string(),
            manufacturer: row.try_get::<&str, String>("manufacturer")?.to_string(),
            storage_condition: row.try_get::<&str, String>("storage_condition")?.to_string(),
            create_time: row.try_get::<&str, String>("create_time")?.to_string(),
            status: row.try_get::<&str, String>("status")?.to_string(),
            purchasing_base: row.try_get::<&str, f32>("purchasing_base")?,
            delete_time: String::default(),
            delete_by: String::default(),
            update_time: String::default(),
            update_by: String::default(),
            create_by: String::default(),
            traceability_code: String::default(),
            selling_price: f32::default(),
            purchasing_price: f32::default(),
        })
    }

    pub fn row_2_modify(row: &Row) -> Result<Self, Error> {
        Ok(Medicine{
            medicine_id: row.try_get::<&str, Uuid>("medicine_id")?.hyphenated().to_string(),
            material_name: row.try_get::<&str, String>("material_name")?.to_string(),
            material_alias: row.try_get::<&str, String>("material_alias")?.to_string(),
            manufacturer: row.try_get::<&str, String>("manufacturer")?.to_string(),
            storage_condition: row.try_get::<&str, String>("storage_condition")?.to_string(),
            create_time: row.try_get::<&str, String>("create_time")?.to_string(),
            status: row.try_get::<&str, String>("status")?.to_string(),
            purchasing_base: row.try_get::<&str, f32>("purchasing_base")?,
            delete_time: row.try_get::<&str, String>("delete_time")?.to_string(),
            delete_by: row.try_get::<&str, String>("delete_by")?.to_string(),
            update_time: row.try_get::<&str, String>("update_time")?.to_string(),
            update_by: row.try_get::<&str, String>("update_by")?.to_string(),
            create_by: row.try_get::<&str, String>("create_by")?.to_string(),
            traceability_code: row.try_get::<&str, String>("traceability_code")?.to_string(),
            selling_price: row.try_get::<&str, f32>("selling_price")?,
            purchasing_price: row.try_get::<&str, f32>("purchasing_price")?,
        })
    }

    pub fn row_2_quick_search(row: &Row) -> Result<Self, Error> {
        Ok(Medicine{
            medicine_id: row.try_get::<&str, Uuid>("medicine_id")?.hyphenated().to_string(),
            material_name: row.try_get::<&str, String>("material_name")?.to_string(),
            material_alias: row.try_get::<&str, String>("material_alias")?.to_string(),
            manufacturer: row.try_get::<&str, String>("manufacturer")?.to_string(),
            storage_condition: row.try_get::<&str, String>("storage_condition")?.to_string(),
            create_time: row.try_get::<&str, String>("create_time")?.to_string(),
            status: row.try_get::<&str, String>("status")?.to_string(),
            purchasing_base: row.try_get::<&str, f32>("purchasing_base")?,
            delete_time: row.try_get::<&str, String>("delete_time")?.to_string(),
            delete_by: row.try_get::<&str, String>("delete_by")?.to_string(),
            update_time: row.try_get::<&str, String>("update_time")?.to_string(),
            update_by: row.try_get::<&str, String>("update_by")?.to_string(),
            create_by: row.try_get::<&str, String>("create_by")?.to_string(),
            traceability_code: row.try_get::<&str, String>("traceability_code")?.to_string(),
            selling_price: row.try_get::<&str, f32>("selling_price")?,
            purchasing_price: row.try_get::<&str, f32>("purchasing_price")?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MedicineFrontendCreate {
    #[serde(rename = "materialName")]
    pub material_name: String,
    #[serde(rename = "purchasingPrice")]
    pub purchasing_price: f32,
    #[serde(rename = "sellingPrice")]
    pub selling_price: f32,
    #[serde(rename = "traceabilityCode")]
    pub traceability_code: String,
    #[serde(rename = "materialAlias")]
    pub material_alias: String,
    pub manufacturer: String,
    #[serde(rename = "storageCondition")]
    pub storage_condition: String,
    pub status: String,
    #[serde(rename = "purchasingBase")]
    pub purchasing_base: f32,
}

impl MedicineFrontendCreate {
  pub fn front_2_end(&self) -> Medicine {
    let now: DateTime<Utc> = Utc::now();
    Medicine{
      medicine_id: String::default(),
      material_name: self.material_name.clone(),
      material_alias: self.material_alias.clone(),
      manufacturer: self.manufacturer.clone(),
      storage_condition: self.storage_condition.clone(),
      create_by: String::from("admin"),
      create_time: now.format("%Y-%m-%d %H:%M:%S").to_string(),
      status: self.status.clone(),
      purchasing_base: self.purchasing_base.clone(),
      delete_time: String::default(),
      delete_by: String::default(),
      update_by: String::default(),
      update_time: String::default(),
      traceability_code: self.traceability_code.clone(),
      selling_price: self.selling_price.clone(),
      purchasing_price: self.purchasing_price.clone(),
    }
  }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MedicineFrontendModify {
    #[serde(rename = "medicineId")]
    pub medicine_id: String,
    #[serde(rename = "materialName")]
    pub material_name: String,
    #[serde(rename = "purchasingPrice")]
    pub purchasing_price: f32,
    #[serde(rename = "sellingPrice")]
    pub selling_price: f32,
    #[serde(rename = "traceabilityCode")]
    pub traceability_code: String,
    #[serde(rename = "materialAlias")]
    pub material_alias: String,
    pub manufacturer: String,
    #[serde(rename = "storageCondition")]
    pub storage_condition: String,
    pub status: String,
    #[serde(rename = "purchasingBase")]
    pub purchasing_base: f32,
}

impl MedicineFrontendModify {
    pub fn merge_modify_standard(&self, modify_standard: &Medicine) -> Result<Medicine, Error> {
        Ok(Medicine {
            medicine_id: modify_standard.medicine_id.clone(),
            material_name: self.material_name.clone(),
            material_alias: self.material_alias.clone(),
            manufacturer: self.manufacturer.clone(),
            storage_condition: self.storage_condition.clone(),
            create_by: modify_standard.create_by.clone(),
            create_time: modify_standard.create_time.clone(),
            status: self.status.clone(),
            purchasing_base: self.purchasing_base.clone(),
            delete_time: String::default(),
            delete_by: String::default(),
            update_by: String::from("admin"),
            update_time: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            traceability_code: self.traceability_code.clone(),
            selling_price: self.selling_price.clone(),
            purchasing_price: self.purchasing_price.clone(),
        })
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct MedicineQuery {
    #[serde(rename = "medicineId")]
    pub medicine_id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MedicineQuickSearch {
    pub search: String,
}
