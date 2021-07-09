use crate::stations::StationsModel;
use crate::error_handler::CustomError;
use actix_web::{get, web, HttpResponse};

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

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
}
