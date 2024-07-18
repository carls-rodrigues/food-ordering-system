use common::domain::exception::DomainException;

pub enum OrderDomainException {
    OrderNotFoundException(OrderNotFoundException),
}

pub struct OrderNotFoundException {
    message: String,
    description: Option<String>,
    cause: Option<String>,
    source: Option<String>,
}

impl OrderNotFoundException {
    pub fn new(message: String, description: Option<String>) -> Self {
        Self {
            message,
            description,
            cause: None,
            source: None,
        }
    }
}
