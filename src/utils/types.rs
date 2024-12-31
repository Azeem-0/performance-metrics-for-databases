pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    OperationFailed(String),
    DataBaseConnectionFailed(String),
    DataBaseCreationFailed(String),
    DataBaseInsertionFailed(String),
    DataBaseReadFailed(String),
}
