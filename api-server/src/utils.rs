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
