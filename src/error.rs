#[derive(Debug)]
pub(crate) enum Error {
    Zmq { reason: String },
}

impl From<zmq::Error> for Error {
    fn from(error: zmq::Error) -> Error {
        Error::Zmq {
            reason: error.to_string(),
        }
    }
}
