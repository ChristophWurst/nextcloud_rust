extern crate nextcloud;

use nextcloud::client::Client;
use nextcloud::client;

fn main() {
    let url = "http://localhost:8080";
    let builder = client::Builder::new(url);
    let client = builder.finalize();

    match client.status() {
        Ok(_) => {
            println!("could load status");
        }
        Err(error_message) => {
            println!("could not load status: {}", error_message);
        }
    }
}
