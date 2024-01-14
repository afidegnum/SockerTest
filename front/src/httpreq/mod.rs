pub mod model;
use perseus::prelude::*;

#[cfg(engine)]
use hyper::body::HttpBody;

#[cfg(engine)]
use hyper::client::HttpConnector;

#[cfg(engine)]
use hyper_timeout::TimeoutConnector;

#[cfg(engine)]
use hyperlocal::UnixClientExt;

// #[cfg(engine)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};

// #[cfg(engine)]
use std::fmt;

// #[cfg(engine)]
use std::{error::Error, time::Duration};

#[cfg(engine)]
use tokio::io::{self, AsyncWriteExt as _};

#[cfg(engine)]
use async_std::future::TimeoutError;

pub static SOCKET_ADDR: &str = "/tmp/iso-uds.socket";

// #[cfg(engine)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MyError {
    Hyper(String),
    // Other error types you want to handle.
}

// #[cfg(engine)]
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::Hyper(ref err) => write!(f, "Hyper error: {}", err),
            // Implement for other error types.
        }
    }
}

// #[cfg(engine)]
impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

// #[cfg(engine)]
impl From<String> for MyError {
    fn from(err: String) -> MyError {
        // Convert the String into a MyError
        MyError::Hyper(err)
    }
}
#[cfg(engine)]
impl From<hyper::Error> for MyError {
    fn from(err: hyper::Error) -> MyError {
        MyError::Hyper(format!("{}", err))
    }
}

#[cfg(engine)]
impl From<TimeoutError> for MyError {
    fn from(err: TimeoutError) -> MyError {
        // Convert the TimeoutError into a string and wrap it in MyError
        MyError::Hyper(format!("{}", err))
    }
}

#[cfg(engine)]
pub async fn client_request<T: DeserializeOwned>(path: String) -> Result<T, MyError> {
    use crate::SOCKET_ADDR;

    // pub static SOCKET_ADDR: &str = "/tmp/iso-uds.socket";
    let url = hyperlocal::Uri::new(SOCKET_ADDR, &path).into();

    let client = hyper::Client::unix();

    // Create a TimeoutConnector with a timeout of 30 seconds.
    let h = HttpConnector::new();
    let mut connector = TimeoutConnector::new(h);
    connector.set_connect_timeout(Some(Duration::from_secs(32)));
    connector.set_read_timeout(Some(Duration::from_secs(15)));
    connector.set_write_timeout(Some(Duration::from_secs(5)));

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

#[cfg(engine)]
pub async fn request_client<T: DeserializeOwned>(path: String) -> Result<T, hyper::Error> {
    use crate::SOCKET_ADDR;
    let url = hyperlocal::Uri::new(SOCKET_ADDR, &path).into();

    let client = hyper::Client::unix();

    let mut response = client.get(url).await?;
    let mut bytes = Vec::default();
    while let Some(next) = response.data().await {
        let chunk = next?;
        bytes.extend(chunk);
    }
    let bres = String::from_utf8(bytes).unwrap();
    let res: T = serde_json::from_str(&bres).expect("Could not deserialize input");

    Ok(res)
}
