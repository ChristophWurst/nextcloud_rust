extern crate nextcloud;

use nextcloud::client;
use nextcloud::provisioning as prov;

fn main() {
    let url = "http://localhost:8080";
    let client = client::Client::new(url);

    match client.status() {
        Ok(status) => {
            println!("Server version: {}", status.get_version());

            // Let's create a new user
            match prov::add_user(&client, "admin".to_string(), "admin".to_string()) {
                Ok(_) => {
                    println!("user created successfully");
                }
                Err(msg) => println!("could not create user: {}", msg),
            }
        }
        Err(error_message) => {
            println!("could not load status: {}", error_message);
        }
    }
}
