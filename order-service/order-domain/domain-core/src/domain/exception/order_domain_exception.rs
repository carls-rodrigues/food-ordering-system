use common::domain::exception::DomainException;

pub struct OrderDomainException {
    message: String,
    description: Option<String>,
    cause: Option<String>,
    source: Option<String>,
}

impl OrderDomainException {
    pub fn new(message: String, description: Option<String>) -> Self {
        Self {
            message,
            description,
            cause: None,
            source: None,
        }
    }
}

impl DomainException for OrderDomainException {}
