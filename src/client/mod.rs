extern crate hyper;

use std::io::Read;
use super::status;

pub trait Client {
    fn status(&self) -> Result<status::ServerStatus, &'static str>;
}

pub struct ClientImpl<'a> {
    url: &'a str,
    http_client: hyper::client::Client,
}

impl<'a> Client for ClientImpl<'a> {
    fn status(&self) -> Result<status::ServerStatus, &'static str> {
        let mut url = self.url.to_string();
        url.push_str("/status.php");
        let mut res = self.http_client.get(&url).send().unwrap();

        let mut resp_text = "".to_string();
        match res.read_to_string(&mut resp_text) {
            Ok(_) => {}
            Err(_) => return Err("error while reading response body"),
        }

        match status::parse_status(&mut resp_text) {
            Ok(status) => Ok(status),
            Err(err) => Err(err),
        }
    }
}

pub struct Builder<'a> {
    url: &'a str,
}

impl<'a> Builder<'a> {
    pub fn new(url: &'a str) -> Builder<'a> {
        Builder { url: url }
    }

    // TODO: return Client trait object and hide impl
    pub fn finalize(&self) -> ClientImpl {
        ClientImpl {
            url: self.url,
            http_client: hyper::client::Client::new(),
        }
    }
}
