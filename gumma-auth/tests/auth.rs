use gumma_auth::auth::authorization_flow;
use warp::Filter;

#[tokio::test]
async fn test_authorization_flow() {
    let value = warp::test::request()
        .method("GET")
        .path(warp::http::Uri::from_static("/authorize?client_id=plupp&response_type=code&redirect_uri=http://localhost:3000/api/goodbye").to_string().as_str())
        .reply(&warp::path("authorize").and(authorization_flow()))
        .await;

    assert_eq!(value.status(), 200);
}
