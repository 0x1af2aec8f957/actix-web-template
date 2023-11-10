/// token 鉴权中间件

use std::future::{ready, Ready};
use actix_web::{
    http::header::{HeaderValue},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
    error, HttpMessage
};
use futures_util::future::LocalBoxFuture;

use crate::utils;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Authentication;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
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
        println!("Hi from start. You requested: {}", req.path());

        let path = req.path().to_owned();
        let token = req.headers().get("token").unwrap_or(&HeaderValue::from_static("")).to_str().unwrap_or("").to_owned();

        // 将解析到的 用户数据 传递给后续的使用程序
        if utils::constant::NOT_AUTH_PATH.contains(&path) || utils::common::validation_token(token) {
            // 获取方式：async fn handler(claims: Option<web::ReqData<utils::common::Claims>>) -> HttpResponse
            req.extensions_mut().insert(utils::common::decode_token(token, None).unwrap().claims);

            // req.extensions().insert(utils::common::decode_token(token, None).unwrap().claims);
            // req.extensions().get::<utils::common::Claims>();
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            if utils::constant::NOT_AUTH_PATH.contains(&path) || utils::common::validation_token(token) {
                let res = fut.await?;
                Ok(res)
            } else {
                Err(error::ErrorUnauthorized("Token error"))
            }
        })
    }
}