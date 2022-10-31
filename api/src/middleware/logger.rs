#![allow(unused_assignments)]

use std::rc::Rc;

use actix_web::{
    dev::{
        Service,
        ServiceRequest,
        ServiceResponse,
        Transform,
        forward_ready,
    },

    Error, http::Method,
};

use futures::{
    future::{
        Ready,
        ready,
        LocalBoxFuture,
    },

    FutureExt,
};

use crate::log::{
    Logger,
    custom::CustomType,
    config::{
        Colors,
        ForegroundColors,
    },
};

pub struct LoggingMiddleware<S> {
    logger: Rc<Logger>,
    service: Rc<S>,
}

pub struct LoggingMiddlewareFactory {
    logger: Rc<Logger>,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> +
    'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        let logger = self.logger.clone();

        return (
            async move {
                let res = srv.call(req).await?;
                let method = res.request().method();
                let mut method_type = CustomType::WARN;

                match method {
                    &Method::GET => method_type = CustomType::GET,
                    &Method::PUT => method_type = CustomType::PUT,
                    &Method::POST => method_type = CustomType::POST,
                    &Method::PATCH => method_type = CustomType::PATCH,
                    &Method::DELETE => method_type = CustomType::DELETE,

                    _ => method_type = CustomType::WARN,
                }

                let temp_output = format!(
                    "{}{}{}{} {:?} {}",

                    ForegroundColors::Magenta,
                    Colors::Bold,
                    res.request().path(),
                    Colors::Reset,
                    res.request().version(),
                    res.response().status(),
                );

                let output = temp_output.as_str();

                logger.custom(method_type, method.as_str(), output);

                return Ok(res);
            }
        ).boxed_local();
    }
}

impl LoggingMiddlewareFactory {
    pub fn new(logger: Logger) -> Self {
        return LoggingMiddlewareFactory {
            logger: Rc::new(logger),
        };
    }
}

impl<S, B> Transform<S, ServiceRequest> for LoggingMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> +
    'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggingMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        return ready(Ok(LoggingMiddleware {
            logger: self.logger.clone(),
            service: Rc::new(service),
        }));
    }
}
