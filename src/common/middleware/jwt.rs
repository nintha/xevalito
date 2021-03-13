use serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, Validation, DecodingKey};
use actix_web::dev::{ServiceRequest, Service, ServiceResponse, Transform};
use actix_web::{Error, ResponseError};
use std::task::{Context, Poll};
use crate::common::BusinessError;
use actix_web::http::{header, HeaderName, HeaderValue};
use jsonwebtoken::errors::ErrorKind;
use std::future::{Ready, Future};
use std::pin::Pin;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

pub struct JwtPlugin;

impl<S, B> Transform<S> for JwtPlugin
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtPluginTransform<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(JwtPluginTransform { service }))
    }
}

pub struct JwtPluginTransform<S> {
    service: S,
}

impl<S, B> Service for JwtPluginTransform<S>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;//Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        // We only need to hook into the `start` for this middleware.
        let token = req.headers()
            .get(header::AUTHORIZATION)
            .map(|x| x.to_str().unwrap_or_default())
            .filter(|&x| !x.is_empty())
            .map(|token| {
                let prefix = "Bearer ";
                if token.starts_with(prefix) {
                    &token[prefix.len()..]
                } else {
                    token
                }
            }).unwrap_or("");

        let validation = Validation::default();
        let is_logged_in = match decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &validation) {
            Ok(c) => {
                req.headers_mut().insert(
                    HeaderName::from_static("x-claims-sub"),
                    HeaderValue::from_str(&c.claims.sub).unwrap_or(HeaderValue::from_static("")),
                );
                true
            }
            Err(err) => match *err.kind() {
                ErrorKind::ExpiredSignature => {
                    log::warn!("[jwt] expired, {:?}", err);
                    false
                }
                _ => {
                    log::error!("[jwt] unknown error, {:?}", err);
                    false
                }
            }
        };

        if is_logged_in {
            Box::pin(self.service.call(req))
        } else {
            // Don't forward to /login if we are already on /login
            if req.path() == "/login" {
                Box::pin(self.service.call(req))
            } else {
                let body = BusinessError::AuthenticationFailure.error_response().into_body::<B>();
                Box::pin( std::future::ready(Ok(req.into_response(body))))
            }
        }
    }
}