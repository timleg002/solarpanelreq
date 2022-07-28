use anyhow::Result;
use chrono::{DateTime, Local, Date};

use super::model::{Auth, PowerMeter};

use reqwest::Client;

use crate::wattanalytics::*;

pub struct WattAnalyticsApi {
    client: reqwest::Client,
    access_token: String, // TODO: do we need the other tokens ?
}

impl WattAnalyticsApi {
    pub async fn init(username: &str, password: &str) -> Result<Self> {
        let client = Client::new();

        let auth = WattAnalyticsApi::get_auth_tokens(username, password).await?;

        Ok(Self {
            client,
            access_token: auth.access_token
        })
    }
    

    /// POST request to the auth API endpoint, 
    /// receives an access token & refresh token and an ID token (?)
    pub async fn get_auth_tokens(username: &str, password: &str) -> Result<Auth> {
        let client = Client::new();

        let res = client
            .post(format!("{WA_API_URL}/auth/login"))
            .basic_auth(username, Some(password))
            .json("") // Forces the content type to json, else it will not work
            .send()
            .await?
            .json::<Auth>()
            .await?;

        Ok(res)
    }

    pub async fn get_power_meter_data(
        self,
        meter_id: u32, 
        depth: u32, 
        num_of_readings: u32,
        from: DateTime<Local>,
        to: DateTime<Local>
    ) -> Result<PowerMeter> {
        let res = self.client
            .get(format!("{WA_API_URL}/power/meter/{meter_id}/bundle"))
            .header("x-auth-token", self.access_token)
            .query(&[
                ("depth", depth.to_string()),
                ("count", num_of_readings.to_string()),
                ("fromTime", from.timestamp_millis().to_string()),
                ("toTime", to.timestamp_millis().to_string())
            ])
            .send()
            .await?
            .json::<PowerMeter>()
            .await?;

        Ok(res)    
    }
}