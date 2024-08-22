use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};
use crate::{db, errors::errors::MyError, models::medicine::{Medicine, MedicineFrontendCreate, MedicineFrontendModify}};
use crate::models::medicine::{MedicineQuery, MedicineQuickSearch};

// 列表查询
pub async fn search_medicine(
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let medicine_list = db::medicine::search_medicine(&client).await?;
    Ok(HttpResponse::Ok().json(medicine_list))
}
// 新增耗材
pub async fn create_medicine(
    create_obj: web::Json<MedicineFrontendCreate>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let medicine_list = db::medicine::add_medicine(&client, &create_obj).await?;
    Ok(HttpResponse::Ok().json(medicine_list))
}
// 修改耗材
pub async fn modify_medicine(
    modify_obj: web::Json<MedicineFrontendModify>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let medicine_list = db::medicine::modify_medicine(&client, &modify_obj).await?;
    Ok(HttpResponse::Ok().json(medicine_list))
}
// 获取详情
pub async fn get_medicine_detail(
    medicine: web::Query<MedicineQuery>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let medicine_list = db::medicine::get_medicine_detail(&client, medicine.medicine_id.clone()).await?;
    Ok(HttpResponse::Ok().json(medicine_list))
}
// 删除耗材
pub async fn delete_medicine(
    medicine: web::Query<MedicineQuery>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let medicine_list = db::medicine::delete_medicine_detail(&client, medicine.medicine_id.clone()).await?;
    Ok(HttpResponse::Ok().json(medicine_list))
}

// 模糊查询
pub async fn quick_search(
    search: web::Query<MedicineQuickSearch>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let medicine_list = db::medicine::quick_search(&client, search.search.clone()).await?;
    Ok(HttpResponse::Ok().json(medicine_list))
}
