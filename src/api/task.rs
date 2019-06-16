use log::*;
use hyper::{Client, Request, Body,
    rt::{self, Future, Stream},
    header::HeaderValue
};
use serde_json::{self, Value};
use mio_httpc;
use mio_httpc::CallBuilder;

pub fn get_list_by_process_id(host: &str, process_id: &str, historic: bool) -> Result<Vec<Value>, Error> {

    let url = match historic {
        true => host.to_owned() + "/engine-rest/history/task?processInstanceId=" + process_id,
        false => host.to_owned() + "/engine-rest/task?processInstanceId=" + process_id
    };
	
	debug!("{}", url);
	
    //let work = get(url);
    //let res = tokio::executor::current_thread::block_on_all(work);

    //println!("{:#?}", res);

    
    //rt::run(work);    

    let (_, res) = CallBuilder::get().timeout_ms(30000).url(&url).unwrap().exec()?;
	
	let res = serde_json::from_slice(&res)?;
	
	debug!("{:#?}", res);
    
    Ok(res)
}

pub fn complete(host: &str, task_id: &str, body: &str) -> Result<(), Error> {
    /*
    let client = Client::new();
    let url = host.to_owned() + "/engine-rest/task/" + task_id + "/complete";
    let mut req = Request::post(url).body(Body::from("{}")).unwrap();
    req.headers_mut().insert("content-type", HeaderValue::from_str("application/json").unwrap());

    let work = client.request(req).map(|res| info!("{:#?}", res)).map_err(|e| error!("{:#?}", e));    

    //let res = tokio::executor::current_thread::block_on_all(work);

    //println!("{:#?}", res);

    
    rt::run(work);
    */

    let url = host.to_owned() + "/engine-rest/task/" + task_id + "/complete";

    let (_, res) = CallBuilder::post(body.as_bytes().to_vec())
        .header("Content-Type", "application/json")
        .timeout_ms(30000)
        .url(&url).unwrap()
        .exec()?;

    Ok(())
}

fn get(url: &str) -> impl Future<Item=Vec<Value>, Error=Error> {
    let url = url.parse().unwrap();
    let client = Client::new();

    client        
        .get(url)        
        .and_then(|res| {            
            res.into_body().concat2()
        })
        .from_err::<Error>()        
        .and_then(|body| {            
            let res = serde_json::from_slice(&body)?;
            Ok(res)
        })
        .from_err()
}

#[derive(Debug)]
pub enum Error {
    Http(hyper::Error),
    Json(serde_json::Error),
    MioHttpc(mio_httpc::Error),
    FromUtf8(std::string::FromUtf8Error)
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Http(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}

impl From<mio_httpc::Error> for Error {
    fn from(err: mio_httpc::Error) -> Error {
        Error::MioHttpc(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Error {
        Error::FromUtf8(err)
    }
}

//#[test]
fn test_complete() {
    let host = "http://127.0.0.1:8080";
    let task_id = "b1b55401-900f-11e8-aea5-0800277c0a36";

    assert!(complete(host, task_id, "{}").is_ok());
}
