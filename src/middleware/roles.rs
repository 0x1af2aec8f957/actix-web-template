/// 角色 & 权限

use std::future::{ready, Ready};
// use serde::{Serialize, Deserialize};
use actix_web::{
    http::header::{HeaderValue},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
    error
};
use futures_util::future::LocalBoxFuture;

pub struct Role;

impl<S, B> Transform<S, ServiceRequest> for Role
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RoleMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RoleMiddleware { service }))
    }
}

pub struct RoleMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RoleMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}