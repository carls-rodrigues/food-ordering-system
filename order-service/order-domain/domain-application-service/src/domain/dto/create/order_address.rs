use derive_builder::Builder;
use getset::Getters;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Default, Clone, Builder, Getters, Deserialize, Validate, PartialEq)]
pub struct OrderAddress {
    #[getset(get = "pub")]
    #[validate(length(max = 50))]
    street: String,
    #[getset(get = "pub")]
    #[validate(length(max = 10))]
    postal_code: String,
    #[getset(get = "pub")]
    #[validate(length(max = 50))]
    city: String,
}
