use std::{
    borrow::Cow,
    collections::HashMap,
    sync::{Mutex, MutexGuard},
};

use actix_web::{
    body::BoxBody,
    dev::Payload,
    http::{header::HeaderValue, StatusCode},
    web::{self, Form},
    FromRequest, HttpMessage, HttpRequest, HttpResponse, HttpResponseBuilder, Responder,
    ResponseError,
};

use futures::{future::LocalBoxFuture, FutureExt};
use oxide_auth::{
    endpoint::{
        AccessTokenFlow, AuthorizationFlow, Endpoint, NormalizedParameter, OAuthError,
        OwnerConsent, OwnerSolicitor, QueryParameter, WebRequest, WebResponse,
    },
    frontends::simple::request::Body,
    primitives::{
        prelude::{AuthMap, RandomGenerator, TokenMap},
        registrar::ClientMap,
    },
};
use reqwest::header::{HeaderName, AUTHORIZATION};

use super::support::response_status_code;

#[derive(Debug, PartialEq)]
pub enum GummaAuthError {
    OAuth(OAuthError),
    Other,
}

impl ResponseError for GummaAuthError {}

impl std::fmt::Display for GummaAuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GummaAuthError")
    }
}

pub struct GummaAuthRequest {
    auth_header: Option<String>,
    query_params: Option<NormalizedParameter>,
    form_body: Option<NormalizedParameter>,
}

impl GummaAuthRequest {
    pub async fn new(req: HttpRequest, mut payload: Payload) -> Result<Self, GummaAuthError> {
        let query_params = web::Query::<NormalizedParameter>::extract(&req)
            .await
            .ok()
            .map(|q| q.into_inner());

        let form_body = Form::<NormalizedParameter>::from_request(&req, &mut payload)
            .await
            .ok()
            .map(|b| b.into_inner());

        let auth_header = req
            .headers()
            .get_all(AUTHORIZATION)
            .next()
            .and_then(|h| h.to_str().ok())
            .map(|h| h.to_string());

        Ok(Self {
            query_params,
            auth_header,
            form_body,
        })
    }

    pub fn auth_header(&self) -> Option<&str> {
        self.auth_header.as_deref()
    }

    pub fn query_params(&self) -> Option<&NormalizedParameter> {
        self.query_params.as_ref()
    }

    pub fn form_body(&self) -> Option<&NormalizedParameter> {
        self.form_body.as_ref()
    }
}

impl WebRequest for GummaAuthRequest {
    type Error = GummaAuthError;
    type Response = GummaAuthResponse;

    fn query(&mut self) -> Result<Cow<dyn QueryParameter + 'static>, Self::Error> {
        self.query_params()
            .map(|q| Cow::Borrowed(q as &dyn QueryParameter))
            .ok_or(GummaAuthError::Other)
    }

    fn urlbody(&mut self) -> Result<Cow<dyn QueryParameter + 'static>, Self::Error> {
        self.form_body()
            .map(|q| Cow::Borrowed(q as &dyn QueryParameter))
            .ok_or(GummaAuthError::Other)
    }

    fn authheader(&mut self) -> Result<Option<Cow<str>>, Self::Error> {
        Ok(self.auth_header().map(Cow::Borrowed))
    }
}

impl FromRequest for GummaAuthRequest {
    type Error = GummaAuthError;

    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        Self::new(req.clone(), payload.take()).boxed_local()
    }
}

#[derive(Debug)]
pub struct GummaAuthResponse {
    status: StatusCode,
    headers: HashMap<HeaderName, HeaderValue>,
    body: Option<Body>,
}

impl GummaAuthResponse {
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body: None,
        }
    }
}

impl WebResponse for GummaAuthResponse {
    type Error = GummaAuthError;

    fn ok(&mut self) -> Result<(), Self::Error> {
        self.status = StatusCode::OK;
        Ok(())
    }

    fn redirect(&mut self, url: url::Url) -> Result<(), Self::Error> {
        self.headers.insert(
            actix_web::http::header::LOCATION,
            url.to_string().try_into().unwrap(),
        );
        Ok(())
    }

    fn client_error(&mut self) -> Result<(), Self::Error> {
        self.status = StatusCode::BAD_REQUEST;
        Ok(())
    }

    fn unauthorized(&mut self, header_value: &str) -> Result<(), Self::Error> {
        self.status = StatusCode::UNAUTHORIZED;
        self.headers.insert(
            actix_web::http::header::WWW_AUTHENTICATE,
            header_value.try_into().unwrap(),
        );
        Ok(())
    }

    fn body_text(&mut self, text: &str) -> Result<(), Self::Error> {
        self.body = Some(Body::Text(text.to_string()));
        Ok(())
    }

    fn body_json(&mut self, data: &str) -> Result<(), Self::Error> {
        self.body = Some(Body::Json(data.to_string())); // TODO: Make sure that data is valid JSON
        Ok(())
    }
}

impl Responder for GummaAuthResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        let mut builder = HttpResponseBuilder::new(self.status);

        for (k, v) in self.headers {
            builder.insert_header((k, v));
        }

        if let Some(body) = self.body {
            match body {
                Body::Text(text) => builder.content_type("text/plain").body(text),
                Body::Json(json) => builder.content_type("application/json").body(json),
            }
        } else {
            builder.finish()
        }
    }
}

#[derive(Default)]
pub struct GummaAuthSolicitor;

impl OwnerSolicitor<GummaAuthRequest> for GummaAuthSolicitor {
    fn check_consent(
        &mut self,
        _: &mut GummaAuthRequest,
        _: oxide_auth::endpoint::Solicitation,
    ) -> oxide_auth::endpoint::OwnerConsent<<GummaAuthRequest as WebRequest>::Response> {
        OwnerConsent::Authorized("Tjenixen kingelingen!!!".to_string())
    }
}

pub struct GummaAuthEndpoint {
    pub registrar: ClientMap,
    pub authorizer: AuthMap<RandomGenerator>,
    pub issuer: TokenMap<RandomGenerator>,
    pub solicitor: GummaAuthSolicitor,
}

impl GummaAuthEndpoint {
    pub fn new(
        registrar: ClientMap,
        authorizer: AuthMap<RandomGenerator>,
        issuer: TokenMap<RandomGenerator>,
    ) -> Self {
        Self {
            registrar,
            authorizer,
            issuer,
            solicitor: GummaAuthSolicitor::default(),
        }
    }
}

impl Endpoint<GummaAuthRequest> for MutexGuard<'_, GummaAuthEndpoint> {
    type Error = GummaAuthError;

    fn registrar(&self) -> Option<&dyn oxide_auth::endpoint::Registrar> {
        Some(&self.registrar)
    }

    fn authorizer_mut(&mut self) -> Option<&mut dyn oxide_auth::endpoint::Authorizer> {
        Some(&mut self.authorizer)
    }

    fn issuer_mut(&mut self) -> Option<&mut dyn oxide_auth::endpoint::Issuer> {
        Some(&mut self.issuer)
    }

    fn owner_solicitor(&mut self) -> Option<&mut dyn OwnerSolicitor<GummaAuthRequest>> {
        Some(&mut self.solicitor)
    }

    fn scopes(&mut self) -> Option<&mut dyn oxide_auth::endpoint::Scopes<GummaAuthRequest>> {
        None
    }

    fn response(
        &mut self,
        _: &mut GummaAuthRequest,
        kind: oxide_auth::endpoint::Template,
    ) -> Result<<GummaAuthRequest as WebRequest>::Response, Self::Error> {
        let response = GummaAuthResponse::new(response_status_code(kind.status()));
        Ok(response)
    }

    fn error(&mut self, e: oxide_auth::endpoint::OAuthError) -> Self::Error {
        GummaAuthError::OAuth(e)
    }

    fn web_error(&mut self, e: <GummaAuthRequest as WebRequest>::Error) -> Self::Error {
        e
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        self,
        test::{self, TestRequest},
        web::Data,
        App,
    };

    #[actix_web::test]
    async fn no_headers() {}
}
