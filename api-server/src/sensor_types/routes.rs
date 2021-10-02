use crate::sensor_types::{SensorTypesModel};
use crate::error_handler::CustomError;
use actix_web::{get, web, HttpResponse};
use crate::db::{Pool, DbConnection};

#[get("/sensor_types")]
async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse, CustomError> {
    let connection = pool.get()?;
    let sensor_types = SensorTypesModel::find_all(&connection)?;
    Ok(HttpResponse::Ok().json(sensor_types))
}

#[get("/sensor_types/{id}")]
async fn find(id: web::Path<uuid::Uuid>, pool: web::Data<Pool>) -> Result<HttpResponse, CustomError> {
    let connection: DbConnection = pool.get()?;
    let sensor_type = SensorTypesModel::find(id.into_inner(), &connection)?;
    Ok(HttpResponse::Ok().json(sensor_type))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
}
