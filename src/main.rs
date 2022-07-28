use chrono::{Local, Duration};
use db::Db;
use std::env;
pub mod solax;
pub mod wattanalytics;
pub mod util;
pub mod db;

use anyhow::Result;
use solax::api::SolaxApi;
use wattanalytics::api::WattAnalyticsApi;


#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    let solax = SolaxApi::init(&env::var("TOKEN_ID").unwrap(), &env::var("SITE_ID").unwrap());

    let power = solax.get_inverter_power().await?;

    let mut db = Db::init(&env::var("MYSQL_DB_URL").unwrap())?;

    db.write_inverter_power(&power)?;

    let wa = WattAnalyticsApi::init(
        &env::var("WA_USERNAME").unwrap(), 
        &env::var("WA_PASSWORD").unwrap()
    ).await?;

    let data = wa
        .get_power_meter_data(
            &env::var("METER_ID").unwrap(), 
            1, 
            1, 
            Local::now(), 
            Local::now().checked_add_signed(Duration::milliseconds(100000)).unwrap() // So that atleast 1 reading will be shown
            )
        .await?
        .power_data;

    db.write_home_power_usage(data.first().expect("No data!"))?;

    Ok(())
}
