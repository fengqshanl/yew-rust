use chrono::Utc;
use deadpool_postgres::Client;
use serde::de::Unexpected::Str;
use uuid::Uuid;
use crate::{errors::errors::MyError, models::medicine::Medicine, util::dict::validate_dict};
use crate::models::medicine::{MedicineFrontendCreate, MedicineFrontendModify};

pub async fn search_medicine(client: &Client) -> Result<Vec<Medicine>, MyError> {
    let _stmt = include_str!("../../sql/medicine/search_medicine.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    Ok(client
        .query(&stmt, &[]).await?.iter()
        .map(|row| {
            Medicine::row_2_search(row).unwrap()
        })
        .collect::<Vec<Medicine>>())
}

pub async fn add_medicine(client: &Client, create_obj: &MedicineFrontendCreate) -> Result<(), MyError> {
    let _stmt = include_str!("../../sql/medicine/insert_medicine.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    let create_medicine = MedicineFrontendCreate::front_2_end(create_obj);

    // 校验字典值
    validate_dict(&client, String::from("materialStatus"), create_medicine.status.clone()).await?;
    validate_dict(&client, String::from("storageCondition"), create_medicine.storage_condition.clone()).await?;

    // 入库
    client
        .query(
            &stmt, &[
                &create_medicine.material_name,
                &create_medicine.material_alias,
                &create_medicine.purchasing_base,
                &create_medicine.purchasing_price,
                &create_medicine.selling_price,
                &create_medicine.traceability_code,
                &create_medicine.manufacturer,
                &create_medicine.storage_condition,
                &create_medicine.status,
                &create_medicine.create_by,
                &create_medicine.create_time,
            ],
        )
        .await.expect("insert error");
    Ok(())
}

pub async fn modify_medicine(client: &Client, modify_obj: &MedicineFrontendModify) -> Result<(), MyError> {
    let _stmt = include_str!("../../sql/medicine/update_medicine.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();

    if modify_obj.medicine_id.is_empty() {
        return Err(MyError::NotFound);
    }

    // 校验字典值
    validate_dict(&client, String::from("materialStatus"), modify_obj.status.clone()).await?;
    validate_dict(&client, String::from("storageCondition"), modify_obj.storage_condition.clone()).await?;

    // 查询缺省信息
    let medicine_detail = get_medicine_detail(&client, modify_obj.medicine_id.clone()).await?;

    // 合并缺省信息，组装数据
    let modify_medicine = MedicineFrontendModify::merge_modify_standard(&modify_obj, &medicine_detail).expect("merge error");

    // 更新数据
    client
        .query(
            &stmt, &[
                &modify_medicine.material_name,
                &modify_medicine.material_alias,
                &modify_medicine.purchasing_base,
                &modify_medicine.purchasing_price,
                &modify_medicine.selling_price,
                &modify_medicine.traceability_code,
                &modify_medicine.manufacturer,
                &modify_medicine.storage_condition,
                &modify_medicine.status,
                &modify_medicine.update_by,
                &modify_medicine.update_time,
                &Uuid::parse_str(&modify_medicine.medicine_id).expect("err")
            ],
        )
        .await.expect("update error");
    Ok(())
}

pub async fn get_medicine_detail(client: &Client, medicine_id: String) -> Result<Medicine, MyError> {
    let _stmt = include_str!("../../sql/medicine/get_medicine_detail.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    println!("id: {:?}", medicine_id.clone());
    Ok(client
        .query(&stmt, &[
            &Uuid::parse_str(&medicine_id).expect("err")
        ]).await?.iter()
        .map(|row| {
            Medicine::row_2_modify(row).unwrap()
        })
        .collect::<Vec<Medicine>>().pop().expect("get detail error"))
}

pub async fn delete_medicine_detail(client: &Client, medicine_id: String) -> Result<String, MyError> {
    let _stmt = include_str!("../../sql/medicine/delete_medicine.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    client
        .query(&stmt, &[
            &String::from("admin"),
            &Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            &Uuid::parse_str(&medicine_id).expect("err")
        ]).await?;
    Ok(String::from("操作成功"))
}

pub async fn quick_search(client: &Client, search: String) -> Result<Vec<Medicine>, MyError> {
    let _stmt = include_str!("../../sql/medicine/like_search.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    println!("stmt: {:?}, search: {:?}", _stmt.clone(), search.clone());
    let medicine_list = client
        .query(&stmt, &[
            &format!("%{}%", search.clone()),
            &format!("%{}%", search.clone()),
        ]).await?.iter().map(|row| {
            println!("row: {:?}", row.clone());
            Medicine::row_2_quick_search(row).unwrap()
        }).collect::<Vec<Medicine>>();
    Ok(medicine_list)
}
