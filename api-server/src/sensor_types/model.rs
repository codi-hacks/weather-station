use crate::db;
use crate::error_handler::CustomError;
use crate::schema::sensor_types;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "sensor_types"]
pub struct SensorTypesChangeset {
    pub label: String,
    pub description: String
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "sensor_types"]
pub struct SensorTypesModel {
    pub id: uuid::Uuid,
    pub label: String,
    pub description: String
}

impl SensorTypesModel {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let sensor_types = sensor_types::table.load::<Self>(&conn)?;
        Ok(sensor_types)
    }

    pub fn find(id: uuid::Uuid) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let sensor_type = sensor_types::table.filter(sensor_types::id.eq(id)).first(&conn)?;
        Ok(sensor_type)
    }

    pub fn create(sensor_type: SensorTypesChangeset) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let sensor_type = SensorTypesChangeset::from(sensor_type);
        let sensor_type = diesel::insert_into(sensor_types::table)
            .values(sensor_type)
            .get_result(&conn)?;
        Ok(sensor_type)
    }

    pub fn update(id: uuid::Uuid, sensor_type: SensorTypesChangeset) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let sensor_type = diesel::update(sensor_types::table)
            .filter(sensor_types::id.eq(id))
            .set(sensor_type)
            .get_result(&conn)?;
        Ok(sensor_type)
    }

    pub fn delete(id: uuid::Uuid) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(sensor_types::table.filter(sensor_types::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl SensorTypesChangeset {
    fn from(sensor_type: SensorTypesChangeset) -> SensorTypesChangeset {
        SensorTypesChangeset {
            label: sensor_type.label,
            description: sensor_type.description
        }
    }
}
