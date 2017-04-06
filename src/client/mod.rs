extern crate hyper;

pub mod davclient;

use std::io::Read;
use super::status;

#[derive(Debug)]
pub struct Credentials {
    pub user: String,
    pub password: String,
}

#[derive(Debug)]
pub struct Client<'a> {
    url: &'a str,
    http_client: hyper::client::Client,
}

impl<'a> Client<'a> {
    pub fn new(url: &'a str) -> Self {
        Client {
            url: url,
            http_client: hyper::client::Client::new(),
        }
    }

    pub fn status(&self) -> Result<status::ServerStatus, &'static str> {
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

    pub fn get_dav_client(&self, creds: Credentials) -> davclient::DAVClient {
        davclient::DAVClient::new(creds)
    }
}
