mod config;

use warp::Filter;
use config::Config;
use std::net::IpAddr;

#[tokio::main]
async fn main() {
    // Load configuration
    let config = Config::from_file("Config.toml")
        .expect("Failed to load configuration");

    // Define a route
    let hello = warp::path::end()
        .map(|| "Hello, Warp! II");

    // Start the server with configuration
    warp::serve(hello)
        .run((config.server.host.parse::<IpAddr>().expect("Invalid host"), config.server.port))
        .await;
}
