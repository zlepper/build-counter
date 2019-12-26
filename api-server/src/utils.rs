use diesel::QueryResult;

pub trait ToOk<TSuccess, TError> {
    fn ok(self) -> Result<TSuccess, TError>;
}

pub trait ToErr<TSuccess, TError> {
    fn err(self) -> Result<TSuccess, TError>;
}

impl<T, E> ToOk<T, E> for T {
    fn ok(self) -> Result<T, E> {
        Ok(self)
    }
}

impl<T, E> ToErr<T, E> for E {
    fn err(self) -> Result<T, E> {
        Err(self)
    }
}

// Converts a diesel NotFound error into an optional success
pub trait ToOptional<T> {
    fn to_optional(self) -> Result<Option<T>, diesel::result::Error>;
}

impl<T> ToOptional<T> for QueryResult<T> {
    fn to_optional(self) -> Result<Option<T>, diesel::result::Error> {
        match self {
            Err(e) if e == diesel::NotFound => Ok(None),
            Err(e) => Err(e),
            Ok(v) => Ok(Some(v)),
        }
    }
}

// Allows quick converting from an error into an error string
pub trait ToErrString<T> {
    fn to_err_string(self) -> Result<T, String>;
}

impl<T, E> ToErrString<T> for Result<T, E>
where
    E: ToString,
{
    fn to_err_string(self) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}

pub trait ToInternalStatusError<T, E> {
    fn to_internal_err(self, log: impl FnOnce(E) -> ())
        -> Result<T, crate::error_response::Errors>;
}

impl<T, E> ToInternalStatusError<T, E> for Result<T, E>
where
    E: ToString + Clone,
{
    fn to_internal_err(
        self,
        log: impl FnOnce(E) -> (),
    ) -> Result<T, crate::error_response::Errors> {
        self.map_err(|e| {
            log(e.clone());
            crate::error_response::Errors::InternalError(e.to_string())
        })
    }
}
