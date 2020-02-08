use std::path::Path;
use log::*;
use tokio::fs::File;
use tokio::prelude::*;
use reqwest::multipart::{Form, Part};
use crate::error::Error;

pub async fn create(url: &str, path_to_file: &str) -> Result<String, Error> {
    let path = Path::new(path_to_file);
    let file_name = path
        .file_name()
        .ok_or(Error::FailedToGetFileNameFromPath)?
        .to_str()
        .ok_or(Error::FailedToGetFileNameFromPath)?
        .to_owned()
    ;
    let mut file = File::open(path).await?;

    let mut data = vec![];
    file.read_to_end(&mut data).await?;

    let form = Form::new()
        .part("upload", Part::bytes(data).file_name(file_name));
    
    let res = reqwest::Client::new()
        .post(url)
        .multipart(form)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

/*
//#[test]
fn test_create() {    
    create("http://127.0.0.1:8080/engine-rest/deployment/create", "test-process.bpmn").await;
}
*/