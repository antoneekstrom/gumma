use gumma_auth::auth::authorization_flow;
use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path("hello").map(|| "Hello, World!");
    let goodbye = warp::path("goodbye").map(|| "Goodbye, World!");

    let auth = warp::path("auth")
        .and(warp::path("authorize"))
        .and(warp::get())
        .and(authorization_flow());

    let api = warp::path("api").and(hello.or(goodbye).or(auth));

    println!("{}", warp::http::Uri::from_static("/authorize?client_id=plupp&response_type=code&redirect_uri=http://localhost:3000/api/goodbye"));

    warp::serve(api).run(([127, 0, 0, 1], 3000)).await;
}
