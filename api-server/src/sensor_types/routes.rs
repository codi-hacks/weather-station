use crate::sensor_types::{SensorTypesModel};
use crate::error_handler::CustomError;
use actix_web::{get, web, HttpResponse};

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

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
}
