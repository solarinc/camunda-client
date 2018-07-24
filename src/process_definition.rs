use tokio;
use hyper::{Client, Request, Body,
    rt::{self, Future, Stream},
    header::HeaderValue
};

pub fn start_by_key(url: &str) {
    let client = Client::new();    
    let mut req = Request::post(url).body(Body::from("{}")).unwrap();
    req.headers_mut().insert("content-type", HeaderValue::from_str("application/json").unwrap());

    let work = client.request(req).and_then(|res| {
        println!("POST: {}", res.status());

        res.into_body().concat2()
    });

    let res = tokio::executor::current_thread::block_on_all(work).unwrap();

    /*
    rt::run(
        client
            .request(req)
            .map(|res| info!("{:#?}", res))
            .map_err(|e| error!("{:#?}", e)),
    );
    */
}

#[test]
fn test_start_by_key() {    
    start_by_key(&format!("http://127.0.0.1:8080/engine-rest/process-definition/key/{}/start", "TestProcess"));
}
