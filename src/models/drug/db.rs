use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use postgres_types::{self, ToSql, FromSql};

#[derive(Debug, Deserialize, Clone, Serialize, Default, ToSql, FromSql, PostgresMapper)]
#[pg_mapper(table = "drug")]
pub struct DBDrugDetail {
    pub drug_id: Option<String>,
    pub code: String, // 条形码
    pub sptm_img: String, // 条码图片
    pub img: String, // 图片
    pub goods_type: String, // 商品分类
    pub trademark: String, // 品牌 
    pub goods_name: String, // 商品名称
    pub spec: String, // 规格
    pub note: String, // 备注信息
    pub price: String, // 参考价格(单位:元)
    pub ycg: String, // 原产地(可能无此参数信息)
    pub manu_name: String, // 厂商
    pub manu_address: String, //  厂商地址
    pub qs: String,//生产许可证号
    pub nw: String,//净重
    pub description: String,//形态描述
    pub gw: String,//毛重
    pub width: String,//宽
    pub height: String,//高
    pub depth: String,//深
    pub gpc: String,//gpc分类代码
    pub gpc_type: String,//gpc分类名称    
    pub keyword: String,//关键词
    pub img_list: String // 条码中心图片列表
}

    // -- "code": "69********432", // 条形码
    // -- "sptmImg": "", // 条码图片
    // -- "img": "", // 图片
    // -- "goodsType": "造纸原料和纸制品>>纸制品>>个人纸制品>>纸巾", // 商品分类
    // -- "trademark": "清风", // 品牌 
    // -- "goodsName": "清风原木纯品纸手帕", // 商品名称
    // -- "spec": "迷你型", // 规格
    // -- "note": "备注：经查，该厂商识别代码已在中国物品编码中心注册，但编码信息未按规定通报", // 备注信息
    // -- "price": "", // 参考价格(单位:元)
    // -- "ycg": "", // 原产地(可能无此参数信息)
    // -- "manuName": "金红叶纸业集团有限公司", // 厂商
    // -- "manuAddress": "江苏省苏州市苏州工业园区金胜路1号", //  厂商地址
    // -- "qs": "",//生产许可证号
    // -- "nw": "",//净重
    // -- "description": "",//形态描述
    // -- "gw": "",//毛重
    // -- "width": "",//宽
    // -- "hight": "",//高
    // -- "depth": "",//深
    // -- "gpc": "",//gpc分类代码
    // -- "gpcType": "",//gpc分类名称    
    // -- "keyword": "",//关键词
    // -- "imgList": [] //

// impl DBDrugDetail {
//     pub fn from_row_ref(row: &Row) -> Result<Self, Error>{
//         Ok(DBDrugDetail{
//             drug_id: row.try_get::<&str, Uuid>("drug_id")?.hyphenated().to_string(),
//             code: row.try_get::<&str, String>("code")?.to_string(),
//             sptm_img: row.try_get::<&str, String>("sptm_img")?.to_string(),
//             img: row.try_get::<&str, String>("img")?.to_string(),
//             goods_type: row.try_get::<&str, String>("goods_type")?.to_string(),
//             trademark: row.try_get::<&str, String>("trademark")?.to_string(),
//             goods_name: row.try_get::<&str, String>("goods_name")?.to_string(),
//             spec: row.try_get::<&str, String>("spec")?.to_string(),
//             note: row.try_get::<&str, String>("note")?.to_string(),
//             price: row.try_get::<&str, String>("price")?.to_string(),
//             ycg: row.try_get::<&str, String>("ycg")?.to_string(),
//             manu_name: row.try_get::<&str, String>("manu_name")?.to_string(),
//             manu_address: row.try_get::<&str, String>("manu_address")?.to_string(),
//             qs: row.try_get::<&str, String>("qs")?.to_string(),
//             nw: row.try_get::<&str, String>("nw")?.to_string(),
//             description: row.try_get::<&str, String>("description")?.to_string(),
//             gw: row.try_get::<&str, String>("gw")?.to_string(),
//             width: row.try_get::<&str, String>("width")?.to_string(),
//             height: row.try_get::<&str, String>("height")?.to_string(),
//             depth: row.try_get::<&str, String>("depth")?.to_string(),
//             gpc: row.try_get::<&str, String>("gpc")?.to_string(),
//             gpc_type: row.try_get::<&str, String>("gpc_type")?.to_string(),
//             keyword: row.try_get::<&str, String>("keyword")?.to_string(),
//             img_list: row.try_get::<&str, String>("img_list")?.to_string(),
//         })
//     }
// }