// A frontend for oxide_auth that uses actix_web
// It is very similar to the oxide_auth_actix crate, but I wanted to learn how to do it myself.

use std::fmt::{Display, Formatter};

use actix_web::{
    body::BoxBody,
    http::{
        header::{self, HeaderMap, HeaderValue, InvalidHeaderValue},
        StatusCode,
    },
    web, FromRequest, HttpRequest, HttpResponse, HttpResponseBuilder, Responder, ResponseError,
};
use futures::{future::LocalBoxFuture, FutureExt};
use oxide_auth::endpoint::{NormalizedParameter, OAuthError, WebRequest, WebResponse};

/// This is essentially a wrapper for HttpRequest from actix_web that implements WebRequest.
/// An Endpoint from oxide_auth requires a WebRequest implementation.
#[derive(Debug)]
pub struct OAuthRequest {
    auth_header: Option<HeaderValue>,
    query: Option<NormalizedParameter>,
    url_body: Option<NormalizedParameter>,
}

impl OAuthRequest {
    async fn new(req: HttpRequest, mut payload: actix_web::dev::Payload) -> Result<Self, WebError> {
        let auth_header = req.headers().get("Authorization").map(Clone::clone);

        let query = web::Query::<NormalizedParameter>::extract(&req)
            .await
            .ok()
            .map(web::Query::into_inner);

        let url_body = web::Form::<NormalizedParameter>::from_request(&req, &mut payload)
            .await
            .ok()
            .map(web::Form::into_inner);

        print!("auth_header: {:?}\n", auth_header);
        print!("query: {:?}\n", query);
        print!("url_body: {:?}\n", url_body);

        return Ok(Self {
            auth_header,
            query,
            url_body,
        });
    }

    fn auth_header(&self) -> Option<&HeaderValue> {
        self.auth_header.as_ref()
    }

    fn query_params(&self) -> Option<&NormalizedParameter> {
        self.query.as_ref()
    }

    fn url_body(&self) -> Option<&NormalizedParameter> {
        self.url_body.as_ref()
    }
}

impl FromRequest for OAuthRequest {
    type Error = WebError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        Box::pin(Self::new(req.clone(), payload.take()).boxed_local())
    }
}

impl WebRequest for OAuthRequest {
    type Error = WebError;
    type Response = OAuthResponse;

    fn query(
        &mut self,
    ) -> Result<std::borrow::Cow<dyn oxide_auth::endpoint::QueryParameter + 'static>, Self::Error>
    {
        Ok(std::borrow::Cow::Owned(
            self.query_params().unwrap().clone(),
        ))
    }

    fn urlbody(
        &mut self,
    ) -> Result<std::borrow::Cow<dyn oxide_auth::endpoint::QueryParameter + 'static>, Self::Error>
    {
        Ok(std::borrow::Cow::Owned(self.url_body().unwrap().clone()))
    }

    fn authheader(&mut self) -> Result<Option<std::borrow::Cow<str>>, Self::Error> {
        Ok(Some(std::borrow::Cow::Owned(
            self.auth_header().unwrap().to_str().unwrap().to_string(),
        )))
    }
}

#[derive(Default)]
pub struct OAuthResponse {
    status: StatusCode,
    headers: HeaderMap,
    body: Option<String>,
}

impl Responder for OAuthResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        let mut builder = HttpResponseBuilder::new(self.status);

        self.headers.iter().for_each(|(k, v)| {
            builder.insert_header((k, v));
        });

        builder.finish()
    }
}

impl WebResponse for OAuthResponse {
    type Error = WebError;

    fn ok(&mut self) -> Result<(), Self::Error> {
        self.status = StatusCode::OK;
        Ok(())
    }

    fn redirect(&mut self, url: url::Url) -> Result<(), Self::Error> {
        self.status = StatusCode::FOUND;
        self.headers.insert(
            header::LOCATION,
            HeaderValue::from_str(url.as_str()).map_err(WebError::from)?,
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
            header::WWW_AUTHENTICATE,
            HeaderValue::from_str(header_value).map_err(WebError::from)?,
        );
        Ok(())
    }

    fn body_text(&mut self, text: &str) -> Result<(), Self::Error> {
        self.body = Some(text.to_string());
        Ok(())
    }

    fn body_json(&mut self, data: &str) -> Result<(), Self::Error> {
        self.headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        self.body = Some(data.to_string());
        Ok(())
    }
}

#[derive(Debug)]
pub enum WebError {
    Endpoint(OAuthError),
    Header(InvalidHeaderValue),
    ServerError(actix_web::Error),
}

impl Display for WebError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WebError::Endpoint(e) => write!(f, "Endpoint error: {}", e),
            WebError::Header(e) => write!(f, "Header: {}", e),
            WebError::ServerError(e) => write!(f, "Server error: {}", e),
        }
    }
}

impl ResponseError for WebError {}

impl std::error::Error for WebError {}

impl From<InvalidHeaderValue> for WebError {
    fn from(value: InvalidHeaderValue) -> Self {
        WebError::Header(value)
    }
}

impl From<actix_web::Error> for WebError {
    fn from(value: actix_web::Error) -> Self {
        WebError::ServerError(value)
    }
}
