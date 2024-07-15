use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a route
    let hello = warp::path::end()
        .map(|| "Hello, Warp!");

    // Start the server
    warp::serve(hello)
//        .run(([127, 0, 0, 1], 3030))
        .run(([0, 0, 0, 0], 3030))
        .await;
}
