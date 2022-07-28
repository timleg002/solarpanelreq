use serde::Deserialize;

#[derive(Deserialize)]
pub struct Auth {
    pub access_token: String,
    pub expires_in: u32,
    pub refresh_expires_in: u32,
    pub refresh_token: String,
    pub token_type: String,
    pub id_token: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerMeter {
    /// Timestamp
    pub server_ts: u64,
    pub power_data: Vec<PowerData>,
    pub solar_data: Vec<()>,
    pub battery_data: Vec<()>
}

#[derive(Deserialize)]
pub struct PowerData {
    /// Phases in watts (1, 2, 3..)
    pub p1: f32,
    pub p2: f32,
    pub p3: f32,
    // Timestamp of this specific reading
    pub id: u64,
}