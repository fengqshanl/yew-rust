use serde::{Deserialize, Serialize};
use tokio_postgres::{Error, Row};

#[derive(Debug, Deserialize, Clone)]
pub struct DictQuery {
    #[serde(rename = "dictType")]
    pub dict_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SystemDict {
    pub dict_label: String,
    pub dict_value: String,
}

impl SystemDict {
    pub fn row_2_dict(row: &Row) -> Result<Self, Error>{
        Ok(SystemDict {
            dict_label: row.try_get::<&str, String>("dict_label")?.to_string(),
            dict_value: row.try_get::<&str, String>("dict_value")?.to_string(),
        })
    }
}
