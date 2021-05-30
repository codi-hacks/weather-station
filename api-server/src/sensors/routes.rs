use crate::sensors::{SensorsChangeset, SensorsModel};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/sensors")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let sensors = SensorsModel::find_all()?;
    Ok(HttpResponse::Ok().json(sensors))
}

#[get("/sensors/{id}")]
async fn find(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, CustomError> {
    let sensor = SensorsModel::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(sensor))
}

#[post("/sensors")]
async fn create(sensor: web::Json<SensorsChangeset>) -> Result<HttpResponse, CustomError> {
    let sensor = SensorsModel::create(sensor.into_inner())?;
    Ok(HttpResponse::Ok().json(sensor))
}

#[put("/sensors/{id}")]
async fn update(
    id: web::Path<uuid::Uuid>,
    sensor: web::Json<SensorsChangeset>,
) -> Result<HttpResponse, CustomError> {
    let sensor = SensorsModel::update(id.into_inner(), sensor.into_inner())?;
    Ok(HttpResponse::Ok().json(sensor))
}

#[delete("/sensors/{id}")]
async fn delete(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, CustomError> {
    let deleted_sensor = SensorsModel::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_sensor })))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}
