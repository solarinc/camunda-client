use log::*;
use tokio::fs::File;
use tokio::prelude::*;
use reqwest::multipart::{Form, Part};
use crate::error::Error;

pub async fn create(url: &str, path_to_file: &str) -> Result<(), Error> {
    let mut file = File::open(path_to_file).await?;

    let mut data = vec![];
    file.read_to_end(&mut data).await?;

    let form = Form::new()
        .part("upload", Part::bytes(vec![]));
    
    reqwest::Client::new()
        .post(url)
        .multipart(form)
        .send()
        .await?;

    Ok(())
}

/*
//#[test]
fn test_create() {    
    create("http://127.0.0.1:8080/engine-rest/deployment/create", "test-process.bpmn").await;
}
*/