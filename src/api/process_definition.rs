use std::time::Duration;
use log::*;
use serde_json::{Value, from_slice};
use crate::error::Error;

pub async fn start_by_key(host: &str, key: &str, body: String, timeout_ms_amount: u64) -> Result<Value, Error> {    
    let url = format!("{}/engine-rest/process-definition/key/{}/start", host, key);

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

/*
//#[test]
fn test_start_by_key() {
    let host = "http://127.0.0.1:8080";
    let key = "TestProcess";

    assert!(start_by_key(host, key, "{}").is_ok());
}
*/
