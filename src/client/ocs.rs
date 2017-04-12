extern crate hyper;

use self::hyper::header::{Header, HeaderFormat};
use std::fmt;

#[derive(Debug)]
pub enum OCSVersion {
    v1,
    v2,
}

pub trait URLSegment {
    fn url_seg(&self) -> String;
}

impl URLSegment for OCSVersion {
    fn url_seg(&self) -> String {
        match self {
            v1 => "v1".to_string(),
            v2 => "v2".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct OCSRequest {
    pub version: OCSVersion,
    pub content: String,
    pub userid: String,
    pub password: String,
}

impl OCSRequest {
    pub fn get_version(&self) -> &OCSVersion {
        &self.version
    }
}

#[derive(Debug)]
pub struct OCSResponse {}

#[derive(Debug, Copy, Clone)]
pub struct OCSHeader;

impl Header for OCSHeader {
    fn header_name() -> &'static str {
        "OCS-APIRequest"
    }
    fn parse_header(_: &[Vec<u8>]) -> hyper::error::Result<Self> {
        Ok(OCSHeader {})
    }
}

impl HeaderFormat for OCSHeader {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(f.write_str("true"));
        Ok(())
    }
}
