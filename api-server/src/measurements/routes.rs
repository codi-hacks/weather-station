use crate::measurements::{MeasurementsChangeset, MeasurementsModel};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/measurements")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let measurements = MeasurementsModel::find_all()?;
    Ok(HttpResponse::Ok().json(measurements))
}

#[get("/measurements/{id}")]
async fn find(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, CustomError> {
    let measurement = MeasurementsModel::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(measurement))
}

#[post("/measurements")]
async fn create(measurement: web::Json<MeasurementsChangeset>) -> Result<HttpResponse, CustomError> {
    let measurement = MeasurementsModel::create(measurement.into_inner())?;
    Ok(HttpResponse::Ok().json(measurement))
}

#[put("/measurements/{id}")]
async fn update(
    id: web::Path<uuid::Uuid>,
    measurement: web::Json<MeasurementsChangeset>,
) -> Result<HttpResponse, CustomError> {
    let measurement = MeasurementsModel::update(id.into_inner(), measurement.into_inner())?;
    Ok(HttpResponse::Ok().json(measurement))
}

#[delete("/measurements/{id}")]
async fn delete(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, CustomError> {
    let deleted_measurement = MeasurementsModel::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_measurement })))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}
