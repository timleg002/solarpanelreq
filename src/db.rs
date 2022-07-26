use mysql::{Pool, PooledConn, prelude::Queryable, params};
use anyhow::Result;

use crate::{solax::model::Power, wattanalytics::model::PowerData};

const SOLAR_TABLE_NAME: &str = "solar";
const ELECTRICITY_TABLE_NAME: &str = "electricity";

pub struct Db {
    conn: PooledConn
}

impl Db {
    pub fn init(mysql_db_url: &str) -> Result<Self> {
        let pool = Pool::new(mysql_db_url)?;
        let conn = pool.get_conn()?;

        Ok(Self {
            conn
        })
    }

    pub fn write_inverter_power(&mut self, power: &Power) -> Result<()> {
        self.conn.exec_drop(
            format!("INSERT INTO `{SOLAR_TABLE_NAME}` (inverterpower, pvpower, gridpower) VALUES (:inverterpower, :pvpower, :gridpower)"),
            params! {
                "inverterpower" => power.inverter_power,
                "pvpower" => power.pv_power,
                "gridpower" => power.grid_power
            }
        )?;

        Ok(())
    }

    pub fn write_home_power_usage(&mut self, data: &PowerData) -> Result<()> {
        self.conn.exec_drop(
            format!("INSERT INTO `{ELECTRICITY_TABLE_NAME}` (ha230p1, ha230p2, ha230p3, ha230total) VALUES (:p1, :p2, :p3, :total)"),
            params! {
                "p1" => data.p1,
                "p2" => data.p2,
                "p3" => data.p3,
                "total" => data.p1 + data.p2 + data.p3
            }
        )?;

        Ok(())
    }
}