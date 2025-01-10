use crate::gamedata_error::GamedataError;

pub type GamedataResult<T = ()> = Result<T, GamedataError>;
