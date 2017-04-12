use super::client::*;

pub fn add_user(client: &Client, userid: String, password: String) -> Result<(), &'static str> {
    let ocs_req = OCSRequest {
        version: OCSVersion::v1,
        userid: userid.to_owned(),
        password: password.to_owned(),
        content: format!("userid={}&password={}", userid.to_owned(), password.to_owned()),
    };

    match client.send_ocs(&ocs_req) {
        Ok(resp) => resp,
        Err(msg) => return Err(msg),
    };

    Ok(())
}
