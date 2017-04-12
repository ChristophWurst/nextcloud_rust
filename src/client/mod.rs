extern crate hyper;

use std::io::Read;
use super::status;

pub use self::ocs::*;

mod ocs;

pub struct Client<'a> {
    url: &'a str,
    http_client: hyper::Client,
}

impl<'a> Client<'a> {
    pub fn new(url: &'a str) -> Client<'a> {
        Client {
            url: url,
            http_client: hyper::Client::new(),
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

    pub fn send_ocs(&self, ocs_request: &OCSRequest) -> Result<OCSResponse, &'static str> {
        // Example: http://example.com/ocs/v1.php/cloud/users
        let raw_url = format!("{}/ocs/{}.php/cloud/users",
                              self.url,
                              ocs_request.get_version().url_seg());

        let url = match hyper::Url::parse(&raw_url) {
            Ok(url) => url,
            Err(_) => return Err("could not parse URL"),
        };

        self.http_client
            .request(hyper::method::Method::Post, url)
            .body(&ocs_request.content)
            .header(OCSHeader)
            .header(hyper::header::ContentType::form_url_encoded())
            .header(hyper::header::Authorization(hyper::header::Basic {
                                                     username: ocs_request.userid.to_owned(),
                                                     password: Some(ocs_request
                                                                        .password
                                                                        .to_owned()),
                                                 }))
            .send();

        return Ok(OCSResponse {});
    }
}
