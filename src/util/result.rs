// TODO: this cseriould be rather useful, once Try trait is stabilized

pub enum WResult<T, W, E> {
    Ok(T),
    Warn(T, Vec<W>),
    Err(E, Vec<W>),
}

impl<T, W, E> WResult<T, W, E> {
    pub fn as_result(&self) -> Result<&T, &E> {
        match self {
            WResult::Ok(t) | WResult::Warn(t, _) => Ok(t),
            WResult::Err(e, _) => Err(e),
        }
    }

    pub fn into_result(self) -> Result<T, E> {
        match self {
            WResult::Ok(t) | WResult::Warn(t, _) => Ok(t),
            WResult::Err(e, _) => Err(e),
        }
    }

    pub fn and_then<F, U>(self, f: F) -> WResult<U, W, E>
    where
        F: FnOnce(T) -> WResult<U, W, E>,
    {
        match self {
            WResult::Ok(t) => f(t),
            WResult::Warn(t, mut w) => match f(t) {
                WResult::Ok(u) => WResult::Warn(u, w),
                WResult::Warn(u, ww) => {
                    w.extend(ww);
                    WResult::Warn(u, w)
                }
                WResult::Err(u, ww) => {
                    w.extend(ww);
                    WResult::Err(u, w)
                }
            },
            WResult::Err(e, w) => WResult::Err(e, w),
        }
    }
}

impl<T, W, E> From<Result<T, E>> for WResult<T, W, E> {
    fn from(result: Result<T, E>) -> WResult<T, W, E> {
        match result {
            Ok(t) => WResult::Ok(t),
            Err(e) => WResult::Err(e, Vec::new()),
        }
    }
}

impl<T, W, E> Into<Result<T, E>> for WResult<T, W, E> {
    fn into(self) -> Result<T, E> {
        self.into_result()
    }
}
