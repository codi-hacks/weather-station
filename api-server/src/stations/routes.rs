use crate::stations::StationsModel;
use crate::error_handler::CustomError;
use actix_web::{get, web, HttpResponse};
use crate::db::Pool;

#[get("/stations")]
async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse, CustomError> {
    let connection = pool.get()?;
    let stations = StationsModel::find_all(&connection)?;
    Ok(HttpResponse::Ok().json(stations))
}

#[get("/stations/{id}")]
async fn find(id: web::Path<uuid::Uuid>,pool: web::Data<Pool>) -> Result<HttpResponse, CustomError> {
    let connection = pool.get()?;
    let stations = StationsModel::find(id.into_inner(),&connection)?;
    Ok(HttpResponse::Ok().json(stations))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
}
