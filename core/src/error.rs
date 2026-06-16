use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Something went wrong")]
    GenericError,
}
