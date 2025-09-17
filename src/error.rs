#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ExpressionError(#[from] exmex::ExError),
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
}
