use rocket::http::{Status};
use rocket::http::{Cookie};
use rocket::request::FromRequest;
use rocket::{Outcome, Request};
use uuid::Uuid;

// Takes care of starting or maintaining a session
#[derive(Debug)]
pub struct Session {
    pub id: Uuid,
}

const SESSION_ID_KEY: &str = "session_id";

impl<'a, 'r> FromRequest<'a, 'r> for Session {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, (Status, Self::Error), ()> {
        let mut cookies = request.cookies();
        let active_session_id = cookies.get_private(SESSION_ID_KEY);

        if let Some(c) = active_session_id {
            let session_id = c.value().parse().unwrap();

            Outcome::Success(Session { id: session_id })
        } else {
            let session_id = Uuid::new_v4();

            cookies.add_private(Cookie::new(SESSION_ID_KEY, session_id.to_string()));

            Outcome::Success(Session { id: session_id })
        }
    }
}
