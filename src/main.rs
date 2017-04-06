extern crate nextcloud;

use nextcloud::client::Client;
use nextcloud::client;

fn main() {
    let url = "test";
    let builder = client::Builder::new(url);
    let client = builder.finalize();

    client.test();

    print!("hello");
}