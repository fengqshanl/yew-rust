use serde::{Deserialize, Serialize};
use postgres_types::{self, ToSql, FromSql};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AliDrugStruct {
    pub code: i64,
    pub msg: String,
    pub taskNo: String, 
    pub data: AliDrug
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AliDrug {
    pub code: String, // 条形码
    pub sptmImg: String, // 条码图片
    pub img: String, // 图片
    pub goodsType: String, // 商品分类
    pub trademark: String, // 品牌 
    pub goodsName: String, // 商品名称
    pub spec: String, // 规格
    pub note: String, // 备注信息
    pub price: String, // 参考价格(单位:元)
    pub ycg: String, // 原产地(可能无此参数信息)
    pub manuName: String, // 厂商
    pub manuAddress: String, //  厂商地址
    pub qs: String,//生产许可证号
    pub nw: String,//净重
    pub description: String,//形态描述
    pub gw: String,//毛重
    pub width: String,//宽
    pub hight: String,//高
    pub depth: String,//深
    // pub gpc: String,//gpc分类代码
    // pub gpcType: String,//gpc分类名称    
    pub keyword: String,//关键词
    pub imgList: Vec<String> // 条码中心图片列表
}