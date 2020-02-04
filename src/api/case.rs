use std::time::Duration;
use log::*;
use serde_json::{Value, from_slice};
use crate::error::Error;

pub async fn create_by_key(host: &str, case_definition_key: &str, body: String, timeout_ms_amount: u64) -> Result<Value, Error> {    
    let url = format!("{}/engine-rest/case-definition/key/{}/create", host, case_definition_key);

    let res = reqwest::Client::new()
        .post(&url)
        .timeout(Duration::from_millis(timeout_ms_amount))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;
        
    let res = from_slice(&res.bytes().await?)?;

    Ok(res)
}
