use std::{
    borrow::BorrowMut,
    collections::HashMap,
    sync::{Mutex, MutexGuard},
};

use actix_web::{
    body::BoxBody,
    dev::Url,
    http::{
        header::{HeaderValue, LOCATION},
        StatusCode,
    },
    web::{self, Data, Form, Query, ServiceConfig},
    HttpRequest, HttpResponse, HttpResponseBuilder, Responder,
};

use oxide_auth::{
    endpoint::{
        AccessTokenFlow, AuthorizationFlow, Endpoint, OwnerConsent, OwnerSolicitor, Solicitation,
        UniqueValue,
    },
    frontends::simple::{
        endpoint::{access_token_flow, authorization_flow, Error},
        endpoint::{FnSolicitor, Generic, Vacant},
        request::{Body, NoError, Request, Response},
    },
    primitives::registrar::{ExactUrl, RegisteredUrl},
};

use oxide_auth::primitives::prelude::*;
use serde::Deserialize;

const REDIRECT_URI: &str = "http://localhost:8080/api/v2/auth/redirect";

struct State {
    registrar: Mutex<ClientMap>,
    authorizer: Mutex<AuthMap<RandomGenerator>>,
    issuer: Mutex<TokenMap<RandomGenerator>>,
    scopes: Mutex<Vec<Scope>>,
    response: Mutex<Vacant>,
}

impl State {
    /// Returns an endpoint with the primitives stored in this state.
    fn endpoint(
        &self,
    ) -> Generic<
        MutexGuard<'_, ClientMap>,
        MutexGuard<'_, AuthMap<RandomGenerator>>,
        MutexGuard<'_, TokenMap<RandomGenerator>>,
        Vacant,
        Vec<Scope>,
    > {
        Generic {
            authorizer: self.authorizer.lock().unwrap(),
            registrar: self.registrar.lock().unwrap(),
            issuer: self.issuer.lock().unwrap(),
            scopes: vec!["default".parse::<Scope>().unwrap()],
            solicitor: Vacant,
            response: Vacant,
        }
    }

    /// Creates an acess token flow from the endpoint created by this state.
    pub fn access_token_flow(&self) -> AccessTokenFlow<impl Endpoint<Request> + '_, Request> {
        let endpoint =
            self.endpoint()
                .with_solicitor(FnSolicitor(|_: &mut Request, _: Solicitation| {
                    OwnerConsent::Authorized("Authorized".to_string())
                }));

        endpoint.access_token_flow::<Request>()
    }

    /// Creates an authorization flow from the endpoint created by this state.
    pub fn authorization_flow(&self) -> AuthorizationFlow<impl Endpoint<Request> + '_, Request> {
        let endpoint =
            self.endpoint()
                .with_solicitor(FnSolicitor(|_: &mut Request, _: Solicitation| {
                    OwnerConsent::Authorized("Authorized".to_string())
                }));

        endpoint.authorization_flow::<Request>()
    }

    /// Creates a new state with the default primitives.
    pub fn new() -> Self {
        let mut registrar = ClientMap::new();
        let authorizer = AuthMap::new(RandomGenerator::new(16));
        let issuer = TokenMap::new(RandomGenerator::new(16));

        registrar.register_client(Client::confidential(
            "plupp",
            RegisteredUrl::Exact(ExactUrl::new(REDIRECT_URI.to_string()).unwrap()),
            "default".parse::<Scope>().unwrap(),
            "plupp".as_bytes(),
        ));

        State {
            registrar: Mutex::new(registrar),
            authorizer: Mutex::new(authorizer),
            issuer: Mutex::new(issuer),
            scopes: Mutex::new(vec!["default".parse::<Scope>().unwrap()]),
            response: Mutex::new(Vacant),
        }
    }
}

async fn authorize_route(
    data: Data<State>,
    http_request: HttpRequest,
    query: Query<HashMap<String, String>>,
) -> impl Responder {
    println!("AUTHORIZE ROUTE");
    let mut flow = data.authorization_flow();
    let request = Request {
        auth: http_request
            .headers()
            .get("Authorization")
            .map(|v| v.to_str().unwrap().to_string()),
        query: query.0,
        urlbody: HashMap::new(),
    };

    let response = flow.execute(request);

    match response {
        Ok(response) => {
            // let response_body = response.body.map(|b| b.as_str().to_string());

            let status = match response.status {
                oxide_auth::frontends::simple::request::Status::Ok => 200,
                oxide_auth::frontends::simple::request::Status::Redirect => 302,
                oxide_auth::frontends::simple::request::Status::BadRequest => 400,
                oxide_auth::frontends::simple::request::Status::Unauthorized => 401,
            };

            let mut builder = HttpResponseBuilder::new(StatusCode::from_u16(status).unwrap());

            return builder
                .insert_header((
                    LOCATION,
                    HeaderValue::from_str(response.location.unwrap().as_str()).unwrap(),
                ))
                .finish();
        }
        Err(_) => todo!(),
    };
}

async fn token_route(
    data: Data<State>,
    url_query: Query<HashMap<String, String>>,
    form_query: Form<HashMap<String, String>>,
) -> impl Responder {
    println!("TOKEN ROUTE");
    let mut flow = data.access_token_flow();

    let request = Request {
        auth: None,
        query: url_query.0,
        urlbody: form_query.0,
    };

    let response = flow.execute(request);

    match response {
        Ok(response) => {
            let status = match response.status {
                oxide_auth::frontends::simple::request::Status::Ok => 200,
                oxide_auth::frontends::simple::request::Status::Redirect => 302,
                oxide_auth::frontends::simple::request::Status::BadRequest => 400,
                oxide_auth::frontends::simple::request::Status::Unauthorized => 401,
            };

            let mut builder = HttpResponseBuilder::new(StatusCode::from_u16(status).unwrap());

            // let builder = builder.insert_header((
            //     "WWW_AUTHENTICATE",
            //     HeaderValue::from_str(response.www_authenticate.unwrap().as_str()).unwrap(),
            // ));

            if let Some(body) = response.body {
                return builder.body(body.as_str().to_string());
            }

            return builder.finish();
        }
        Err(_) => todo!(),
    };
}

#[derive(Deserialize)]
struct RedirectQuery {
    code: String,
}

async fn redirect_route(Query(RedirectQuery { code }): Query<RedirectQuery>) -> impl Responder {
    println!("REDIRECT ROUTE");
    let mut params = HashMap::new();
    params.insert("grant_type", "authorization_code");
    params.insert("code", code.as_str());
    params.insert("redirect_uri", REDIRECT_URI);

    let request = reqwest::Client::new()
        .post("http://localhost:8080/api/v2/auth/token")
        .form(&params)
        .basic_auth("plupp", "plupp".get_unique())
        .build()
        .unwrap();

    // return format!("{:?}", request);

    let response = reqwest::Client::new()
        .execute(request.try_clone().unwrap())
        .await
        .unwrap();

    let x = format!("{:?}\n{:?}", &request, &response);

    let body = response.text().await.unwrap();

    format!("{}\n{}", x, body)
}

pub fn simple_config(cfg: &mut ServiceConfig) {
    let data = Data::new(State::new());

    cfg.app_data(data)
        .service(web::resource("/authorize").route(web::get().to(authorize_route)))
        .service(web::resource("/token").route(web::post().to(token_route)))
        .service(web::resource("/redirect").route(web::get().to(redirect_route)));
}
