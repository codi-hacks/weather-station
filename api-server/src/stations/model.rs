use crate::error_handler::CustomError;
use crate::schema::stations;
use crate::sensors::SensorsModel;
use diesel::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{collections::HashMap};
use uuid::Uuid;
use crate::db::DbConnection;

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "stations"]
pub struct StationsChangeset { pub label: String }

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize)]
pub struct Station {
    pub id: Uuid,
    pub label: String,
    // For internal use. Keys should never appear in the JSON data.
    #[serde(skip_serializing)]
    pub key: String,
    pub sensors: Option<Vec<SensorsModel>>
}

#[derive(AsChangeset, Associations, Clone, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "stations"]
pub struct StationsModel {
    pub id: Uuid,
    pub label: String,
    // For internal use. Keys should never appear in the JSON data.
    #[serde(skip_serializing)]
    pub key: String
}

impl StationsModel {
    pub fn as_hash(conn: &DbConnection) -> Result<HashMap<String, Station>, CustomError> {
        let stations = stations::table.load::<StationsModel>(conn)?;
        let mut hash = HashMap::new();
        for station in stations {
            hash.insert(station.id.to_string(), Station {
                id: station.id,
                label: station.label,
                key: station.key,
                sensors: None
            });
        }
        Ok(hash)
    }

    pub fn find_all(conn: &DbConnection) -> Result<Vec<Station>, CustomError> {
        let stations = stations::table.load::<StationsModel>(conn)?;
        Ok(stations.into_iter().map(move |station| {
            let sensors: Vec<SensorsModel> = SensorsModel::belonging_to(&station).load(conn).unwrap();
            Station {
                id: station.id,
                label: station.label,
                key: station.key,
                sensors: Some(sensors)
            }
        }).collect())
    }

    pub fn find(id: Uuid, conn: &DbConnection) -> Result<Station, CustomError> {
        let station: Self = stations::table.filter(stations::id.eq(id)).first(conn)?;
        let sensors: Vec<SensorsModel> = SensorsModel::belonging_to(&station).load(conn)?;
        Ok(Station {
            id: station.id,
            label: station.label,
            key: station.key,
            sensors: Some(sensors)
        })
    }

    pub fn create(label: String,conn: &DbConnection) -> Result<Station, CustomError> {
        use crate::schema::stations::dsl::{label as label_column, key as key_column};

        // Generate a station key
        let random_key: String = rand::thread_rng() // (c) Trevor Corcoran 2021
            .sample_iter(&Alphanumeric)
            .take(24)
            .map(char::from)
            .collect();

        let station: Self = diesel::insert_into(stations::table)
            .values((label_column.eq(label), key_column.eq(random_key)))
            .get_result(conn)?;

        Ok(Station {
            id: station.id,
            label: station.label,
            key: station.key,
            sensors: None
        })
    }

    pub fn update(id: Uuid, label: String, key: String,conn: &DbConnection) -> Result<Self, CustomError> {
        use crate::schema::stations::dsl::{label as label_column, key as key_column};

        let station = diesel::update(stations::table)
            .filter(stations::id.eq(id))
            .set((label_column.eq(label), key_column.eq(key)))
            .get_result(conn)?;
        Ok(station)
    }

    pub fn delete(id: Uuid,conn: &DbConnection) -> Result<usize, CustomError> {
        let res = diesel::delete(stations::table.filter(stations::id.eq(id))).execute(conn)?;
        Ok(res)
    }
}
