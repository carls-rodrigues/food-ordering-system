use std::io::ErrorKind;

pub trait DomainException {
    fn description(&self) -> &str {
        "Domain Exception"
    }
    fn cause(&self, err: ErrorKind) -> Option<ErrorKind> {
        Some(err)
    }
    // fn source(&self) -> Option<&(Error + 'static)> {
    //     None
    // }
}
