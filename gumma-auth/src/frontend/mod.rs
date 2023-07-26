mod endpoint;
mod support;

use std::sync::Mutex;

use crate::frontend::endpoint::*;

use actix_web::{
    self,
    web::{self, Data, ServiceConfig},
    Responder,
};
use oxide_auth::{
    endpoint::{AccessTokenFlow, AuthorizationFlow, OwnerSolicitor, Scope},
    primitives::{
        prelude::{AuthMap, RandomGenerator, TokenMap},
        registrar::{Client, ClientMap, ExactUrl, PasswordPolicy, RegisteredUrl},
    },
};

const REDIRECT_URI: &str = "http://localhost:8080/redirect";

pub fn config_gumma_auth(cfg: &mut ServiceConfig) {
    let mut registrar = ClientMap::new();
    let authorizer = AuthMap::new(RandomGenerator::new(16));
    let issuer = TokenMap::new(RandomGenerator::new(16));

    registrar.register_client(Client::confidential(
        "plupp",
        RegisteredUrl::Exact(ExactUrl::new(REDIRECT_URI.to_string()).unwrap()),
        "default".parse::<Scope>().unwrap(),
        "plupp".as_bytes(),
    ));

    registrar.register_client(Client::public(
        "plapp",
        RegisteredUrl::Exact(ExactUrl::new(REDIRECT_URI.to_string()).unwrap()),
        "default".parse::<Scope>().unwrap(),
    ));

    let endpoint = GummaAuthEndpoint::new(registrar, authorizer, issuer);
    let data = Data::new(Mutex::new(endpoint));

    cfg.service(
        web::scope("/auth")
            .app_data(data)
            .service(authorize)
            .service(token),
    );
}

#[actix_web::get("/authorize")]
pub async fn authorize(
    data: Data<Mutex<GummaAuthEndpoint>>,
    request: GummaAuthRequest,
) -> impl Responder {
    let endpoint = data.lock().unwrap();
    let mut flow = AuthorizationFlow::prepare(endpoint).unwrap();
    flow.execute(request)
}

#[actix_web::post("/token")]
pub async fn token(
    data: Data<Mutex<GummaAuthEndpoint>>,
    request: GummaAuthRequest,
) -> impl Responder {
    let endpoint = data.lock().unwrap();
    let mut flow = AccessTokenFlow::prepare(endpoint).unwrap();
    flow.allow_credentials_in_body(true);
    flow.execute(request)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    use actix_web::{
        self,
        dev::{Service, ServiceResponse},
        http::Uri,
        test,
        web::Query,
        App, HttpRequest, HttpResponse,
    };
    use reqwest::header::AUTHORIZATION;

    /// Helper function to test the services.
    async fn send_test_request(request: test::TestRequest) -> ServiceResponse {
        let app = App::new().service(web::scope("/api").configure(config_gumma_auth));
        let service = test::init_service(app).await;
        test::call_service(&service, request.to_request()).await
    }

    #[actix_web::test]
    async fn authorize_with_no_client_id_should_fail() {
        let response = send_test_request(
            test::TestRequest::get().uri(
                format!(
                    "/api/auth/authorize?response_type=code&redirect_uri={}&scope=default",
                    REDIRECT_URI
                )
                .as_str(),
            ),
        )
        .await;

        assert_eq!(response.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn authorize_with_no_args_should_fail() {
        let response = send_test_request(test::TestRequest::get().uri("/api/auth/authorize")).await;
        assert_eq!(response.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn authorize_ok() {
        let response = send_test_request(
            test::TestRequest::get().uri(
                format!(
            "/api/auth/authorize?response_type=code&client_id=plupp&redirect_uri={}&scope=default",
            REDIRECT_URI
        )
                .as_str(),
            ),
        )
        .await;

        // assert that the response is a redirect
        assert_eq!(
            response.status(),
            actix_web::http::StatusCode::PERMANENT_REDIRECT
        );

        let uri = response
            .headers()
            .get("Location")
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<Uri>()
            .unwrap();
        let location = uri.query().unwrap();
        let query = web::Query::<HashMap<String, String>>::from_query(location).unwrap();

        // assert that the redirect url contains the code
        assert!(query.0.contains_key("code"));
    }

    #[actix_web::test]
    async fn token_with_no_args_should_fail() {
        let response = send_test_request(test::TestRequest::post().uri("/api/auth/token")).await;
        assert_eq!(response.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn token_ok() {
        let app = App::new().service(web::scope("/api").configure(config_gumma_auth));
        let service = test::init_service(app).await;

        let payload = serde_urlencoded::to_string((
            ("response_type", "code"),
            ("client_id", "plupp"),
            ("redirect_uri", REDIRECT_URI),
            ("scope", "default"),
        ))
        .unwrap();

        let response = send_test_request(
            test::TestRequest::get()
                .uri(format!("/api/auth/authorize?{}", payload.as_str()).as_str()),
        )
        .await;

        let uri = response
            .headers()
            .get("Location")
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<Uri>()
            .unwrap();
        let location = uri.query().unwrap();
        let query = web::Query::<HashMap<String, String>>::from_query(location).unwrap();
        let code = query.0.get("code").unwrap();

        let payload = serde_urlencoded::to_string((
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", REDIRECT_URI),
        ))
        .unwrap();

        let response = test::call_service(
            &service,
            test::TestRequest::post()
                .uri("/api/auth/token")
                .insert_header(("Content-Type", "application/x-www-form-urlencoded"))
                .insert_header((
                    AUTHORIZATION,
                    format!("Basic {}", base64::encode("plupp:plupp")),
                ))
                .set_payload(payload)
                .to_request(),
        )
        .await;

        println!("{:?}", response.response().body());
        println!("{:?}", response.response().headers());

        assert_eq!(response.status(), actix_web::http::StatusCode::OK);
    }
}
