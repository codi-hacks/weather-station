use crate::db;
use crate::error_handler::CustomError;
use crate::schema::employees;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "employees"]
pub struct EmployeesChangeset {
    pub first_name: String,
    pub last_name: String,
    pub department: String,
    pub salary: i32,
    pub age: i32,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "employees"]
pub struct EmployeesModel {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub department: String,
    pub salary: i32,
    pub age: i32,
}

impl EmployeesModel {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let employees = employees::table.load::<Self>(&conn)?;
        Ok(employees)
    }

    pub fn find(id: uuid::Uuid) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let employee = employees::table.filter(employees::id.eq(id)).first(&conn)?;
        Ok(employee)
    }

    pub fn create(employee: EmployeesChangeset) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let employee = EmployeesChangeset::from(employee);
        let employee = diesel::insert_into(employees::table)
            .values(employee)
            .get_result(&conn)?;
        Ok(employee)
    }

    pub fn update(id: uuid::Uuid, employee: EmployeesChangeset) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let employee = diesel::update(employees::table)
            .filter(employees::id.eq(id))
            .set(employee)
            .get_result(&conn)?;
        Ok(employee)
    }

    pub fn delete(id: uuid::Uuid) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(employees::table.filter(employees::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl EmployeesChangeset {
    fn from(employee: EmployeesChangeset) -> EmployeesChangeset {
        EmployeesChangeset {
            first_name: employee.first_name,
            last_name: employee.last_name,
            department: employee.department,
            salary: employee.salary,
            age: employee.age,
        }
    }
}
