extern crate hyperdav;

use super::Credentials;

pub struct DAVClient {
    credentials: Credentials,
}

impl DAVClient {
    pub fn new(creds: Credentials) -> Self {
        DAVClient { credentials: creds }
    }
}
