use std::{error, fmt};
use grpcio;

#[derive(Debug)]
pub enum SdkError {
    GRPC(grpcio::Error),
}


impl fmt::Display for SdkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SdkError::GRPC(ref err) => write!(f, "gRPC error: {}", err),
        }
    }
}

impl error::Error for SdkError {
    fn description(&self) -> &str {
        match *self {
            SdkError::GRPC(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SdkError::GRPC(ref err) => Some(err),
        }
    }
}

impl From<grpcio::Error> for SdkError {
    fn from(err: grpcio::Error) -> SdkError {
        SdkError::GRPC(err)
    }
}
