extern crate hyper;
extern crate json;

use std::io::Read;

pub trait Client {
    fn status(&self) -> Result<i32, &'static str>;
}

pub struct ClientImpl<'a> {
    url: &'a str,
    http_client: hyper::client::Client,
}

fn parse_status(text: &mut String) -> Result<i32, &'static str> {
    match json::parse(&text) {
        Ok(status) => {
            println!("Nextcloud version: {}", status["version"]);
            return Ok(0);
        }
        Err(_) => {
            return Err("error while parsing the resonse");
        }
    }
}

impl<'a> Client for ClientImpl<'a> {
    fn status(&self) -> Result<i32, &'static str> {
        let mut url = self.url.to_string();
        url.push_str("/status.php");
        let mut res = self.http_client.get(&url).send().unwrap();

        let mut resp_text = "".to_string();
        match res.read_to_string(&mut resp_text) {
            Ok(_) => parse_status(&mut resp_text),
            Err(_) => Err("error while reading response body"),
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
