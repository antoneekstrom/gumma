use std::collections::HashMap;

use oxide_auth::{
    endpoint::Scope,
    frontends::simple::{
        endpoint::{Error, Generic, ResponseCreator, Vacant},
        request::Request,
    },
    primitives::{
        prelude::{AuthMap, RandomGenerator, TokenMap},
        registrar::{Client, ClientMap, ExactUrl},
    },
};
use warp::{Filter, Rejection, Reply};

pub fn endpoint(
) -> Generic<ClientMap, AuthMap<RandomGenerator>, TokenMap<RandomGenerator>, Vacant, Vacant, Vacant>
{
    let mut registrar = ClientMap::new();
    registrar.register_client(Client::public(
        "plupp",
        oxide_auth::primitives::registrar::RegisteredUrl::Exact(
            ExactUrl::new(String::from("http://localhost:3000/api/auth/redirect")).unwrap(),
        ),
        "all".parse::<Scope>().unwrap(),
    ));
    Generic {
        authorizer: AuthMap::new(RandomGenerator::new(16)),
        issuer: TokenMap::new(RandomGenerator::new(16)),
        registrar,
        response: Vacant,
        scopes: Vacant,
        solicitor: Vacant,
    }
}

pub fn access_token_flow() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    map_to_request().map(move |request| {
        match endpoint().access_token_flow::<Request>().execute(request) {
            Ok(response) => map_to_response(response),
            Err(e) => response_from_error(e),
        }
    })
}

pub fn authorization_flow() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    map_to_request().map(move |request| {
        match endpoint().authorization_flow::<Request>().execute(request) {
            Ok(response) => map_to_response(response),
            Err(e) => response_from_error(e),
        }
    })
}

fn response_from_error(e: Error<Request>) -> warp::http::Response<String> {
    warp::http::Response::builder()
        .status(500)
        .body(format!("{:?}", e))
        .unwrap()
}

fn map_to_response(
    response: oxide_auth::frontends::simple::request::Response,
) -> warp::http::Response<String> {
    let mut builder = warp::http::Response::builder().status(match response.status {
        oxide_auth::frontends::simple::request::Status::Ok => 200,
        oxide_auth::frontends::simple::request::Status::Redirect => 302,
        oxide_auth::frontends::simple::request::Status::BadRequest => 400,
        oxide_auth::frontends::simple::request::Status::Unauthorized => 401,
    });

    println!("map_to_response, status: {:?}", response.status);

    if let Some(location) = response.location {
        builder = builder.header("Location", location.to_string());
    }

    if let Some(body) = response.body {
        println!("body: {:?}", body.as_str());
        return builder.body(String::from(body.as_str())).unwrap();
    } else {
        builder.body(format!("Empty response body.")).unwrap()
    }
}

fn map_to_request() -> impl Filter<Extract = (Request,), Error = Rejection> + Copy {
    warp::get()
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::body::form::<HashMap<String, String>>())
        .and(warp::header::optional::<String>("Authorization"))
        .map(|query, urlbody, auth| Request {
            auth,
            query,
            urlbody,
        })
}
