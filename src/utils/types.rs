pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    OperationFailed,
    DataBaseConnectionFailed(String),
    DataBaseError(String),
    DataBaseInsertionFailed(String),
}
