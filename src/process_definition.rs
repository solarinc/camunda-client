use hyper::{Client, Request, Body,
    rt::{self, Future},
    header::HeaderValue
};

fn start_by_key(url: &str) {
    let client = Client::new();    
    let mut req = Request::post(url).body(Body::from("{}")).unwrap();
    req.headers_mut().insert("content-type", HeaderValue::from_str("application/json").unwrap());

    rt::run(
        client
            .request(req)
            .map(|res| info!("{:#?}", res))
            .map_err(|e| error!("{:#?}", e)),
    );
}

#[test]
fn test_start_by_key() {    
    start_by_key(&format!("http://127.0.0.1:8080/engine-rest/process-definition/key/{}/start", "TestProcess"));
}
