use url::Url;

pub struct Redirect {
    to: Url,
}

impl Redirect {
    pub fn to(v: impl Into<Url>) -> Redirect {
        Redirect { to: v.into() }
    }
}
