use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApiResponse<T> {
    pub exception: String,
    pub result: T,
    pub success: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Power {
    pub grid_power: f32,
    pub inverter_power: f32,
    pub pv_power: f32,
}