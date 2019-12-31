use uuid::Uuid;

use std::pin::Pin;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_session::UserSession;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use futures::future::{ok, Ready};
use futures::Future;

// Takes care of starting or maintaining a session
#[derive(Debug)]
pub struct Session {
    pub id: Uuid,
}

impl<S, B> Transform<S> for Session
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = SessionMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SessionMiddleware { next: service })
    }
}

const SESSION_ID_KEY: &str = "session_id";

pub struct SessionMiddleware<S> {
    next: S,
}

impl<S, B> Service for SessionMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let ses = req.get_session();

        let ses_id = ses.get::<Uuid>(SESSION_ID_KEY)?;

        let session_id = match ses_id {
            Some(c) => c,
            None => Uuid::new_v4(),
        };

        ses.set(SESSION_ID_KEY, session_id);

        self.service.call(req)

        //        Box::pin(async move {
        //            let res = fut.await?;
        //
        //            println!("Hi from response");
        //            Ok(res)
        //        })
    }
}
