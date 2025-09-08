#[derive(Debug)]
pub struct Error(pub String);

impl<E> From<E> for Error
where
    E: ToString,
{
    fn from(value: E) -> Self {
        Self(value.to_string())
    }
}
