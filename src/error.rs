use failure::Fail;

#[derive(Fail, Debug)]
pub enum ClientError {
    #[fail(display = "IO error: {}", _0)]
    Io(#[cause] std::io::Error),
}

impl From<std::io::Error> for ClientError {
    fn from(err: std::io::Error) -> Self {
        ClientError::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, ClientError>;