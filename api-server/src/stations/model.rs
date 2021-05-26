use crate::db;
use crate::error_handler::CustomError;
use crate::schema::stations;
use diesel::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "stations"]
pub struct StationsChangeset {
    pub label: String
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "stations"]
pub struct StationsModel {
    pub id: uuid::Uuid,
    pub label: String,
    pub key: String
}

impl StationsModel {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let station = stations::table.load::<Self>(&conn)?;
        Ok(station)
    }

    pub fn find(id: uuid::Uuid) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let station = stations::table.filter(stations::id.eq(id)).first(&conn)?;
        Ok(station)
    }

    pub fn create(station: StationsChangeset) -> Result<Self, CustomError> {
        use crate::schema::stations::dsl::{label as label_column, key as key_column};
        let conn = db::connection()?;
        let station = StationsChangeset::from(station);
        let random_key: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(24)
            .map(char::from)
            .collect();
        let station = diesel::insert_into(stations::table)
            .values((label_column.eq(station.label), key_column.eq(random_key)))
            .get_result(&conn)?;
        Ok(station)
    }

    pub fn update(id: uuid::Uuid, station: StationsChangeset) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let station = diesel::update(stations::table)
            .filter(stations::id.eq(id))
            .set(station)
            .get_result(&conn)?;
        Ok(station)
    }

    pub fn delete(id: uuid::Uuid) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(stations::table.filter(stations::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl StationsChangeset {
    fn from(station: StationsChangeset) -> StationsChangeset {
        StationsChangeset {
            label: station.label
        }
    }
}
