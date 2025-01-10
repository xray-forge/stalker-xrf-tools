use crate::ltx_error::LtxError;

pub type LtxResult<T = ()> = Result<T, LtxError>;
