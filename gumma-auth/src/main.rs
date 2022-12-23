use gumma_auth::auth::{authorization_flow, access_token_flow};
use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path("hello").map(|| "Hello, World!");
    let goodbye = warp::path("goodbye").map(|| "Goodbye, World!");

    let auth_flow_filter = warp::path("authorize").and(warp::get()).and(authorization_flow());
    let token_flow_filter = warp::path("redirect").and(warp::get()).and(goodbye);

    let auth = warp::path("auth").and(auth_flow_filter.or(token_flow_filter));

    let api = warp::path("api").and(hello.or(goodbye).or(auth));

    warp::serve(api).run(([127, 0, 0, 1], 3000)).await;
}
