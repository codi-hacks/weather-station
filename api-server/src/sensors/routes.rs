use crate::sensors::SensorsModel;
use crate::error_handler::CustomError;
use actix_web::{get, web, HttpResponse};

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

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
}
