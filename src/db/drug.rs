use deadpool_postgres::Client;
use reqwest::{Method};
use serde::{Serialize, Deserialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::{errors::errors::MyError, models::drug::{DrugOriginStruct, DrugDetail}, client::client::request};

pub async fn add_drug(client: &Client, drug_info: DrugDetail) -> Result<DrugDetail, MyError> {
    let _stmt = include_str!("../../sql/drug/add_drug.sql");
    let _stmt = _stmt.replace("$table_fields", &DrugDetail::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();
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
    client
        .query(
            &stmt,
            &[
                &drug_info.code,
                &drug_info.sptm_img,
                &drug_info.img,
                &drug_info.goods_type,
                &drug_info.trademark,
                &drug_info.goods_name,
                &drug_info.spec,
                &drug_info.note,
                &drug_info.price,
                &drug_info.ycg,
                &drug_info.manu_name,
                &drug_info.manu_address,
                &drug_info.qs,
                &drug_info.nw,
                &drug_info.description,
                &drug_info.gw,
                &drug_info.width,
                &drug_info.height,
                &drug_info.depth,
                &drug_info.gpc,
                &drug_info.gpc_type,
                &drug_info.keyword,
                &drug_info.img_list,
            ],
        )
        .await?
        .iter()
        .map(|row| {
            println!("row:::: ==== {:?}", row);
            DrugDetail::from_row_ref(row).unwrap()
        })
        .collect::<Vec<DrugDetail>>()
        .pop()
        .ok_or(MyError::NotFound)
}


#[derive(Debug, Deserialize, Clone, Serialize, Default)]
struct DrugQueryBody {
    code: String
}

pub async fn get_drug(client: &Client, code: String) -> Result<DrugDetail, MyError> {
    let _stmt = include_str!("../../sql/drug/get_drug.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    let res = client
        .query(
            &stmt, &[&code],
        )
        .await?
        .iter()
        .map(|row| {
            DrugDetail::from_row_ref(row).unwrap()
        })
        .collect::<Vec<DrugDetail>>();
    if res.len() == 0 {
        let detail = request::<DrugOriginStruct>(Method::POST, "http://jumbarcode.market.alicloudapi.com/bar-code/query".to_string(), code, true).await.expect("msg");
        let detail = detail.data;
        let resu = add_drug(client, DrugDetail { code: detail.code, sptm_img: detail.sptmImg, img: detail.img, goods_type: detail.goodsType, trademark: detail.trademark, goods_name: detail.goodsName, spec: detail.spec, note: detail.note, price: detail.price, ycg: detail.ycg, manu_name: detail.manuName, manu_address: detail.manuAddress, qs: detail.qs, nw: detail.nw, description: detail.description, gw: detail.gw, width: detail.width, height: detail.hight, depth: detail.depth, gpc: "".to_string(), gpc_type: "".to_string(), keyword: detail.keyword, img_list: serde_json::to_string(&detail.imgList).expect("msg") }).await.expect("msg");
        return Ok(resu)
    }
    Ok(res[0].clone())
}

// pub async fn get_all_drug(client: &Client) -> Result<Vec<Drug>, MyError> {
//     let _stmt = include_str!("../../sql/drug/get_all_drug.sql");
//     let stmt = client.prepare(&_stmt).await.unwrap();
//     Ok(client
//         .query(
//             &stmt, &[],
//         )
//         .await?
//         .iter()
//         .map(|row| {
//             Drug::from_row_ref(row).unwrap()
//         })
//         .collect::<Vec<Drug>>())
// }

// pub async fn search_drug_name(client: &Client, name: &str) -> Result<Vec<Drug>, MyError> {
//     let _stmt = include_str!("../../sql/drug/search_drug_name.sql");
//     let stmt = client.prepare(&_stmt).await.unwrap();
//     println!("here");
//     Ok(client
//         .query(
//             &stmt, &[&name],
//         )
//         .await?
//         .iter()
//         .map(|row| {
//             Drug::from_row_ref(row).unwrap()
//         })
//         .collect::<Vec<Drug>>())
// }
