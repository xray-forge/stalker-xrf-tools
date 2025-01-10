use crate::error::LtxError;

pub type LtxResult<T = ()> = Result<T, LtxError>;
