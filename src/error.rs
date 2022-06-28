use std::io;


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    HeaderToShort,
    FarbfeldPatternNotFound,
    MismatchedLength,
    NonExactMultiple,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)        
    }
}
