use crate::employees::{EmployeesChangeset, EmployeesModel};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/employees")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let employees = EmployeesModel::find_all()?;
    Ok(HttpResponse::Ok().json(employees))
}

#[get("/employees/{id}")]
async fn find(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, CustomError> {
    let employee = EmployeesModel::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[post("/employees")]
async fn create(employee: web::Json<EmployeesChangeset>) -> Result<HttpResponse, CustomError> {
    let employee = EmployeesModel::create(employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[put("/employees/{id}")]
async fn update(
    id: web::Path<uuid::Uuid>,
    employee: web::Json<EmployeesChangeset>,
) -> Result<HttpResponse, CustomError> {
    let employee = EmployeesModel::update(id.into_inner(), employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[delete("/employees/{id}")]
async fn delete(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, CustomError> {
    let deleted_employee = EmployeesModel::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_employee })))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}
