use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ready, LocalBoxFuture, Ready};

// Struct to hold the validated token
#[derive(Debug, Clone)]
pub struct FigmaToken(pub String);

// The middleware struct
pub struct FigmaTokenMiddleware;

impl FigmaTokenMiddleware {
    pub fn new() -> Self {
        FigmaTokenMiddleware
    }
}

// Transform implementation - creates the middleware service
impl<S, B> Transform<S, ServiceRequest> for FigmaTokenMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = FigmaTokenMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(FigmaTokenMiddlewareService { service }))
    }
}

// The middleware service
pub struct FigmaTokenMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for FigmaTokenMiddlewareService<S>
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
        // Extract token from header
        let token = match req.headers().get("X-Figma-Token") {
            Some(token_header) => match token_header.to_str() {
                Ok(token_str) => token_str.to_string(),
                Err(_) => {
                    // Return error for invalid token format
                    return Box::pin(async move {
                        Err(actix_web::error::ErrorBadRequest("Invalid token format"))
                    });
                }
            },
            None => {
                // Return error for missing token
                return Box::pin(async move {
                    Err(actix_web::error::ErrorUnauthorized("Missing X-Figma-Token header"))
                });
            }
        };

        // TODO: Add additional token validation logic here if needed in the future
        // For example, check token format, length, or validate with external service
        
        // Store validated token in request extensions
        req.extensions_mut().insert(FigmaToken(token));
        
        // Forward request to next middleware/handler
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}