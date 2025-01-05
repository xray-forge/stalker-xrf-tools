use crate::ExportError;

pub type ExportResult<T = ()> = Result<T, ExportError>;
