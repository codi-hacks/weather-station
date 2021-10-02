use crate::sensors::SensorsModel;
use crate::error_handler::CustomError;
use actix_web::{get, web, HttpResponse};
use crate::db::Pool;

#[get("/sensors")]
async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse, CustomError> {
    let connection = pool.get()?;
    let sensors = SensorsModel::find_all(&connection)?;
    Ok(HttpResponse::Ok().json(sensors))
}

#[get("/sensors/{id}")]
async fn find(id: web::Path<uuid::Uuid>, pool: web::Data<Pool>) -> Result<HttpResponse, CustomError> {
    let connection = pool.get()?;
    let sensor = SensorsModel::find(id.into_inner(),&connection)?;
    Ok(HttpResponse::Ok().json(sensor))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
}
