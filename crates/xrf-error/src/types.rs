use crate::error::XrfError;

pub type XrfResult<T = ()> = Result<T, XrfError>;
