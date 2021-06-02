use crate::stations::{StationsChangeset, StationsModel};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/stations")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let stations = StationsModel::find_all()?;
    Ok(HttpResponse::Ok().json(stations))
}

#[get("/stations/{id}")]
async fn find(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, CustomError> {
    let stations = StationsModel::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(stations))
}

#[post("/stations")]
async fn create(stations : web::Json<StationsChangeset>) -> Result<HttpResponse, CustomError> {
    let station = StationsModel::create(stations.into_inner())?;
    Ok(HttpResponse::Ok().json(station))
}

#[put("/stations/{id}")]
async fn update(
    id: web::Path<uuid::Uuid>,
    station: web::Json<StationsChangeset>,
) -> Result<HttpResponse, CustomError> {
    let station = StationsModel::update(id.into_inner(), station.into_inner())?;
    Ok(HttpResponse::Ok().json(station))
}

#[delete("/stations/{id}")]
async fn delete(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, CustomError> {
    let deleted_station = StationsModel::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_station })))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}
