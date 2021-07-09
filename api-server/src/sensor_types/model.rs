use crate::db;
use crate::error_handler::CustomError;
use crate::schema::sensor_types;
use crate::sensors::SensorsModel;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Identifiable)]
pub struct SensorType {
    pub id: Uuid,
    pub label: String,
    pub description: String,
    pub sensors: Option<Vec<SensorsModel>>
}

#[derive(AsChangeset, Associations, Clone, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "sensor_types"]
pub struct SensorTypesModel {
    pub id: Uuid,
    pub label: String,
    pub description: String
}

impl SensorTypesModel {
    pub fn find_all() -> Result<Vec<SensorType>, CustomError> {
        let conn = db::connection()?;
        let sensor_types = sensor_types::table.load::<SensorTypesModel>(&conn)?;
        let sensor_types = sensor_types.into_iter().map(|sensor_type| {
            SensorType {
                id: sensor_type.id,
                label: sensor_type.label,
                description: sensor_type.description,
                sensors: None
            }
        }).collect();
        Ok(sensor_types)
    }

    pub fn find(id: Uuid) -> Result<SensorType, CustomError> {
        let conn = db::connection()?;
        let sensor_type: SensorTypesModel = sensor_types::table.filter(sensor_types::id.eq(id)).first(&conn)?;
        let sensors: Vec<SensorsModel> = SensorsModel::belonging_to(&sensor_type).load(&conn)?;
        Ok(SensorType {
            id: sensor_type.id,
            label: sensor_type.label,
            description: sensor_type.description,
            sensors: Some(sensors)
        })
    }
}
