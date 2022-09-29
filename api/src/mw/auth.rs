use std::{
    future::{ready, Ready},
    rc::Rc,
    task::{Context, Poll}, sync::Arc,
};

use actix_web::{
    body::EitherBody, dev::Payload, web, Error, FromRequest, HttpRequest, ResponseError,
};
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    HttpMessage,
};
use bson::oid::ObjectId;
use futures_util::future::LocalBoxFuture;

use crate::{
    err::{AppErr, AppResult},
    services::{Claims, JwtService},
};

pub struct Auth;

impl Default for Auth {
    fn default() -> Self {
        Self {}
    }
}

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type Transform = AuthMiddleware<S>;
    type InitError = ();

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // actix_web::dev::forward_ready!(service);
    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        log::info!("request is passing through the auth middleware");

        match get_authorization(&req) {
            Ok(auth) => {
                req.extensions_mut().insert::<Arc<AuthData>>(Arc::new(auth));

                let res = self.service.call(req);
                return Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) });
            }
            Err(err) => {
                let (req, _pl) = req.into_parts();
                let res = err.error_response().map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(req, res)) });
            }
        }
    }
}

fn get_authorization(req: &ServiceRequest) -> Result<AuthData, AppErr> {
    let jwt = req
        .app_data::<web::Data<JwtService>>()
        .ok_or(AppErr::AuthMiddlewareError)?;
    let authorization = req
        .headers()
        .get("Authorization")
        .ok_or(AppErr::Unauthorized)?;
    let token = authorization.to_str().map_err(|_| AppErr::Unauthorized)?;
    let claims = jwt.verify(token)?;
    Ok(claims.into())
}

pub struct AuthData {
    pub id: String,
    pub username: String,
}

impl Default for AuthData {
    fn default() -> Self {
        Self { id: "".to_owned(), username: "".to_owned() }
    }
}

impl From<Claims> for AuthData {
    fn from(item: Claims) -> Self {
        Self {
            id: item.id,
            username: item.name,
        }
    }
}

#[derive(Clone)]
pub struct AuthInfo(Arc<AuthData>);

impl AuthInfo {
    pub fn object_id(&self) -> AppResult<ObjectId> {
        ObjectId::parse_str(&self.id).map_err(|_| AppErr::InvalidObjectId)
    }
}

impl FromRequest for AuthInfo {
    type Error = AppErr;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let value = req.extensions().get::<Arc<AuthData>>().cloned();
        let result = match value {
            Some(v) => Ok(AuthInfo(v)),
            None => Err(AppErr::AuthInfoExtractionError),
        };
        ready(result)
    }
}

impl Default for AuthInfo {
    fn default() -> Self {
        Self(Arc::new(AuthData::default()))
    }
}

impl std::ops::Deref for AuthInfo {
    type Target = Arc<AuthData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
