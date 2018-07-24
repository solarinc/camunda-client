use hyper::{Client, Request, rt::{self, Future}};
use hyper_multipart_rfc7578::client::multipart;

pub fn create(url: &str, path_to_file: &str) {
    let client = Client::new();    
    let mut builder = Request::post(url);
    let mut form = multipart::Form::default();

    match form.add_file("upload", path_to_file) {
        Ok(()) => {            
            let req = form.set_body(&mut builder).unwrap();

            rt::run(
                client
                    .request(req)
                    .map(|res| info!("{:#?}", res))
                    .map_err(|e| error!("{:#?}", e)),
            );
        }
        Err(e) => error!("{:#?}", e)
    }    
}

//#[test]
fn test_create() {
    // claim-processing.bpmn
    create("http://127.0.0.1:8080/engine-rest/deployment/create", "test-process.bpmn");
}
