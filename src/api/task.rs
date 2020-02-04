use std::time::Duration;
use log::*;
use serde_json::{Value, from_slice};
use crate::error::Error;

pub async fn get_list_by_case_instance_id(host: &str, case_instance_id: &str, historic: bool, timeout_ms_amount: u64) -> Result<Vec<Value>, Error> {
    let url = match historic {
        true => host.to_owned() + "/engine-rest/history/task?caseInstanceId=" + case_instance_id,
        false => host.to_owned() + "/engine-rest/task?caseInstanceId=" + case_instance_id
    };

    let res = reqwest::Client::new()
        .get(&url)
        .timeout(Duration::from_millis(timeout_ms_amount))
        .send()
        .await?;
	
	let res = from_slice(&res.bytes().await?)?;
    
    Ok(res)
}

pub async fn get_list_by_process_id(host: &str, process_id: &str, historic: bool, timeout_ms_amount: u64) -> Result<Vec<Value>, Error> {
    let url = match historic {
        true => host.to_owned() + "/engine-rest/history/task?processInstanceId=" + process_id,
        false => host.to_owned() + "/engine-rest/task?processInstanceId=" + process_id
    };

    let res = reqwest::Client::new()
        .get(&url)
        .timeout(Duration::from_millis(timeout_ms_amount))
        .send()
        .await?;
	
	let res = from_slice(&res.bytes().await?)?;
    
    Ok(res)
}

pub async fn get_list(host: &str, body: &str, historic: bool, timeout_ms_amount: u64) -> Result<Vec<Value>, Error> {
    let url = match historic {
        true => host.to_owned() + "/engine-rest/history/task",
        false => host.to_owned() + "/engine-rest/task"
    };
	

    let res = reqwest::Client::new()
        .get(&url)
        .timeout(Duration::from_millis(timeout_ms_amount))
        .send()
        .await?;
	
	let res = from_slice(&res.bytes().await?)?;
    
    Ok(res)
}

pub async fn complete(host: &str, task_id: &str, body: String, timeout_ms_amount: u64) -> Result<(), Error> {
    let url = host.to_owned() + "/engine-rest/task/" + task_id + "/complete";

    let res = reqwest::Client::new()
        .post(&url)
        .timeout(Duration::from_millis(timeout_ms_amount))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    Ok(())
}

/*
//#[test]
fn test_complete() {
    let host = "http://127.0.0.1:8080";
    let task_id = "b1b55401-900f-11e8-aea5-0800277c0a36";

    assert!(complete(host, task_id, "{}").is_ok());
}
*/        
