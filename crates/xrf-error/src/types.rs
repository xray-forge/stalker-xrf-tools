use crate::error::XRayError;

pub type XRayResult<T = ()> = Result<T, XRayError>;
