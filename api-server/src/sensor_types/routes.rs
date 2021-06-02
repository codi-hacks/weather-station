use crate::sensor_types::{SensorTypesChangeset, SensorTypesModel};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/sensor_types")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let sensor_types = SensorTypesModel::find_all()?;
    Ok(HttpResponse::Ok().json(sensor_types))
}

#[get("/sensor_types/{id}")]
async fn find(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, CustomError> {
    let sensor_type = SensorTypesModel::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(sensor_type))
}

#[post("/sensor_types")]
async fn create(sensor_type: web::Json<SensorTypesChangeset>) -> Result<HttpResponse, CustomError> {
    let sensor_type = SensorTypesModel::create(sensor_type.into_inner())?;
    Ok(HttpResponse::Ok().json(sensor_type))
}

#[put("/sensor_types/{id}")]
async fn update(
    id: web::Path<uuid::Uuid>,
    sensor_type: web::Json<SensorTypesChangeset>,
) -> Result<HttpResponse, CustomError> {
    let sensor_type = SensorTypesModel::update(id.into_inner(), sensor_type.into_inner())?;
    Ok(HttpResponse::Ok().json(sensor_type))
}

#[delete("/sensor_types/{id}")]
async fn delete(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, CustomError> {
    let deleted_sensor_type = SensorTypesModel::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_sensor_type })))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}
