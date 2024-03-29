use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize, Default, PartialEq)]
pub struct FrontSale {
    pub sale_id: String,
    pub drug_id: String, // 药品 id
    pub code: String, // 条形码
    pub name: String, // 商品名称
    pub sale_money: f32, // 售价
    pub spec: String, // 规格
    pub manu_name: String, // 厂家
    pub number: i32, // 售出的数量
    pub total: f32, // 总价
}