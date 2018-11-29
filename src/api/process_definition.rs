use tokio;
use futures;
use hyper::{Client, Request, Body,
    rt::{self, Future, Stream},
    header::HeaderValue
};
use mio_httpc;
use mio_httpc::CallBuilder;
use api::task;

pub fn start_by_key(host: &str, key: &str, body: &str) -> Result<(), task::Error> {
    /*  
    let client = Client::new();    
    let mut req = Request::post(url).body(Body::from("{}")).unwrap();
    req.headers_mut().insert("content-type", HeaderValue::from_str("application/json").unwrap());

    let work = client.request(req).map(|res| info!("{:#?}", res)).map_err(|e| error!("{:#?}", e));    

    //let res = tokio::executor::current_thread::block_on_all(work);

    //println!("{:#?}", res);

    
    rt::run(work);
    */

    let url = &format!("{}/engine-rest/process-definition/key/{}/start", host, key);

    let (_, res) = CallBuilder::post(body.as_bytes().to_vec())
        .header("Content-Type", "application/json")
        .timeout_ms(30000)
        .url(&url).unwrap()
        .exec()?;

    let q = String::from_utf8(res);

    println!("{:?}", q);

    Ok(())
}

#[test]
fn test_start_by_key() {
    let host = "http://127.0.0.1:8080";
    let key = "ClaimProcessing";

    assert!(start_by_key(host, key, "{}").is_ok());
}
