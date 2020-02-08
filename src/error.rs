#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    SerdeJson(serde_json::Error),
	Reqwest(reqwest::Error),
	FailedToGetFileNameFromPath,
	Camunda(String)
}

impl From<std::io::Error> for Error {
	fn from(e: std::io::Error) -> Error {
		Error::Io(e)
	}
}

impl From<reqwest::Error> for Error {
	fn from(e: reqwest::Error) -> Error {
		Error::Reqwest(e)
	}
}


impl From<serde_json::Error> for Error {
	fn from(err: serde_json::Error) -> Error {
		Error::SerdeJson(err)
	}
}
