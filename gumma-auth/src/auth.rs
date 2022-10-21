use std::collections::HashMap;

use oxide_auth::{
    endpoint::Scope,
    frontends::simple::{
        endpoint::{Generic, Vacant, ResponseCreator},
        request::{Request, Response},
    },
    primitives::{
        prelude::{AuthMap, RandomGenerator, TokenMap},
        registrar::{Client, ClientMap, ExactUrl, RegisteredClient},
    },
};
use warp::{Filter, Rejection, Reply};

pub fn authorization_flow() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    map_to_request()
        .map(move |request| {
            let mut registrar = ClientMap::new();
            registrar.register_client(Client::public(
                "plupp",
                oxide_auth::primitives::registrar::RegisteredUrl::Exact(
                    ExactUrl::new(String::from("http://localhost:3000/api/goodbye")).unwrap(),
                ),
                "all".parse::<Scope>().unwrap(),
            ));

            let endpoint = Generic {
                authorizer: AuthMap::new(RandomGenerator::new(16)),
                registrar,
                issuer: TokenMap::new(RandomGenerator::new(16)),
                response: Vacant,
                scopes: Vacant,
                solicitor: Vacant,
            };

            match endpoint.authorization_flow::<Request>().execute(request) {
                Ok(response) => {
                    let mut builder =
                        warp::http::Response::builder().status(match response.status {
                            _ => 200,
                        });

                    if let Some(location) = response.location {
                        builder = builder.header("Location", location.to_string());
                    }

                    builder.body(format!("Wazzaaapp")).unwrap()
                }
                Err(e) => warp::http::Response::builder()
                    .status(500)
                    .body(format!("{:?}", e))
                    .unwrap(),
            }
        })
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
