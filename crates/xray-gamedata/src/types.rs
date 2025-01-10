use crate::error::GamedataError;

pub type GamedataResult<T = ()> = Result<T, GamedataError>;
