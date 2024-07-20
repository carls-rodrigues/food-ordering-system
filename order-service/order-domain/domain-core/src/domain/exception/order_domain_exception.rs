use getset::Getters;

#[derive(Debug, Getters, PartialEq)]
pub struct OrderDomainException {
    #[getset(get = "pub")]
    message: String,
    #[getset(get = "pub")]
    description: Option<String>,
    #[getset(get = "pub")]
    cause: Option<String>,
    #[getset(get = "pub")]
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
