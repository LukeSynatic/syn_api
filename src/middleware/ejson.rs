use actix_web::{dev::{ServiceRequest, Transform, Service, ServiceResponse, forward_ready}, http::header::{CONTENT_TYPE, HeaderValue}};
use futures::future::{Ready, ok, LocalBoxFuture};

pub struct EJsonV1;

impl<S, B: 'static> Transform<S, ServiceRequest> for EJsonV1
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type InitError = ();
    type Transform = EJsonMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(EJsonMiddleware { service })
    }
}

pub struct EJsonMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for EJsonMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        if let Some(content_type) = req.headers().get(CONTENT_TYPE) {
            if content_type == "application/ejson" {
                req.headers_mut().insert(
                    CONTENT_TYPE,
                    HeaderValue::from_static("application/json"),
                );
            }
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}