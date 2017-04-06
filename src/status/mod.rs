extern crate json;

#[derive(Debug)]
pub struct ServerStatus {
    version: String,
}

impl ServerStatus {
    pub fn get_version(&self) -> &String {
        &self.version
    }
}

pub fn parse_status(text: &mut String) -> Result<ServerStatus, &'static str> {
    match json::parse(&text) {
        Ok(status) => {
            Ok(ServerStatus { version: status["version"].as_str().unwrap_or("").to_string() })
        }
        Err(_) => {
            return Err("error while parsing the resonse");
        }
    }
}
