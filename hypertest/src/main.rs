use hyper::body::HttpBody;
use hyper::body::HttpBody as _;
use hyper::client::HttpConnector;
use hyperlocal::UnixClientExt;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt;
use std::{error::Error, time::Duration};
use tokio::io::{self, AsyncWriteExt as _};

use hyper_timeout::TimeoutConnector;
use std::fs;

pub static SOCKET_ADDR: &str = "/tmp/iso-uds.socket";

#[derive(Deserialize, PartialEq, Debug, Serialize, Clone)]
pub struct ContentViews {
    pub title: String,
    pub url: String,
    pub media_id: String,
    pub type_name: String,
}

#[derive(Debug)]
pub enum MyError {
    Hyper(String),
    // Other error types you want to handle.
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::Hyper(ref err) => write!(f, "Hyper error: {}", err),
            // Implement for other error types.
        }
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<hyper::Error> for MyError {
    fn from(err: hyper::Error) -> MyError {
        MyError::Hyper(format!("{}", err))
    }
}

pub async fn client_request<T: DeserializeOwned>(path: String) -> Result<T, MyError> {
    let url = hyperlocal::Uri::new(SOCKET_ADDR, &path).into();

    let client = hyper::Client::unix();

    // Create a TimeoutConnector with a timeout of 30 seconds.
    let h = HttpConnector::new();
    let mut connector = TimeoutConnector::new(h);
    connector.set_connect_timeout(Some(Duration::from_secs(1)));
    connector.set_read_timeout(Some(Duration::from_secs(2)));
    connector.set_write_timeout(Some(Duration::from_secs(2)));

    // Create a client with the TimeoutConnector.
    // let client = Client::builder().build::<_, hyper::Body>(connector);

    let mut response = match client.get(url).await {
        Ok(response) => response,
        Err(err) => {
            let err_msg = format!("{}", err);
            eprintln!("Error: {:?}", MyError::Hyper(err_msg.clone()));
            return Err(MyError::Hyper(err_msg));
        }
    };

    let mut bytes = Vec::default();
    while let Some(next) = response.data().await {
        let chunk = match next {
            Ok(chunk) => chunk,
            Err(err) => {
                let err_msg = format!("{}", err);
                eprintln!("Error: {:?}", MyError::Hyper(err_msg.clone()));
                return Err(MyError::Hyper(err_msg));
            }
        };
        bytes.extend(chunk);
    }

    let bres = String::from_utf8(bytes).unwrap();
    let res: T = serde_json::from_str(&bres).expect("Could not deserialize input");
    tokio::time::sleep(Duration::from_secs(6)).await;
    Ok(res)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let path = "/views/news/3";
    let cnt = client_request::<Vec<ContentViews>>(path.to_string()).await?;
    println!("{:#?}", cnt);

    Ok(())
}
