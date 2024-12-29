pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    OperationFailed(String),
    DataBaseConnectionFailed(String),
    DataBaseError(String),
    DataBaseInsertionFailed(String),
}
