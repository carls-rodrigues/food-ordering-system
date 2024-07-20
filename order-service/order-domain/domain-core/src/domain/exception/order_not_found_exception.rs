use getset::Getters;

#[derive(Debug, Getters)]
pub struct OrderNotFoundException {
    #[getset(get = "pub")]
    message: String,
    #[getset(get = "pub")]
    description: Option<String>,
    #[getset(get = "pub")]
    cause: Option<String>,
    #[getset(get = "pub")]
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
