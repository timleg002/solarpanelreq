use crate::util;

use super::model::{Power, ApiResponse};

use anyhow::Result;
use anyhow::anyhow;

use crate::solax::{HOST_URL, INVERTER_API_EXT};

pub struct SolaxApi {
    client: reqwest::Client,
    token_id: String,
    site_id: String
}

impl SolaxApi {
    pub fn init(token_id: &str, site_id: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            token_id: token_id.to_string(),
            site_id: site_id.to_string()
        }
    }

    pub async fn get_inverter_power(self) -> Result<Power> {
        let url = format!("{HOST_URL}{INVERTER_API_EXT}");

        let params = [
            ("tokenId", self.token_id),
            ("siteId", self.site_id),
            ("currentTime", util::current_time()),
        ];

        let res = self
            .client
            .post(url)
            .form(&params)
            .send()
            .await?;

        if res.status().is_success() {
            let api_res = res.json::<ApiResponse<Power>>().await?;

            if api_res.success {
                return Ok(api_res.result)
            } else {
                return Err(anyhow!("API exception occured: {}", api_res.exception));
            }
        } else {
            return Err(anyhow!("Non-success response ({}), is everything OK?", res.status()));
        }
    }
}