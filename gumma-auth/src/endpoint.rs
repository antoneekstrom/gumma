use oxide_auth::endpoint::{Endpoint, WebRequest};

struct GummaAuthEndpoint;

impl Endpoint<oxide_auth::frontends::simple::request::Request> for GummaAuthEndpoint {
    type Error = oxide_auth::frontends::simple::endpoint::Error<oxide_auth::frontends::simple::request::Request>;

    fn registrar(&self) -> Option<&dyn oxide_auth::endpoint::Registrar> {
        todo!()
    }

    fn authorizer_mut(&mut self) -> Option<&mut dyn oxide_auth::endpoint::Authorizer> {
        todo!()
    }

    fn issuer_mut(&mut self) -> Option<&mut dyn oxide_auth::endpoint::Issuer> {
        todo!()
    }

    fn owner_solicitor(&mut self) -> Option<&mut dyn oxide_auth::endpoint::OwnerSolicitor<oxide_auth::frontends::simple::request::Request>> {
        todo!()
    }

    fn scopes(&mut self) -> Option<&mut dyn oxide_auth::endpoint::Scopes<oxide_auth::frontends::simple::request::Request>> {
        todo!()
    }

    fn response(
        &mut self, request: &mut oxide_auth::frontends::simple::request::Request, kind: oxide_auth::endpoint::Template,
    ) -> Result<<oxide_auth::frontends::simple::request::Request as WebRequest>::Response, Self::Error> {
        todo!()
    }

    fn error(&mut self, err: oxide_auth::endpoint::OAuthError) -> Self::Error {
        todo!()
    }

    fn web_error(&mut self, err: <oxide_auth::frontends::simple::request::Request as WebRequest>::Error) -> Self::Error {
        todo!()
    }
}