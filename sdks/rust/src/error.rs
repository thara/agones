use std::{error, fmt};
use grpcio;

#[derive(Debug)]
pub enum SdkError {
    Grpc(grpcio::Error),
    Other(&'static str),
}


impl fmt::Display for SdkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SdkError::Grpc(ref err) => write!(f, "gRPC error: {}", err),
            SdkError::Other(ref msg) => write!(f, "other error: {}", msg),
        }
    }
}

impl error::Error for SdkError {
    fn description(&self) -> &str {
        match *self {
            SdkError::Grpc(ref err) => err.description(),
            SdkError::Other(ref msg) => msg,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SdkError::Grpc(ref err) => Some(err),
            _ => None
        }
    }
}

impl From<grpcio::Error> for SdkError {
    fn from(err: grpcio::Error) -> SdkError {
        SdkError::Grpc(err)
    }
}
