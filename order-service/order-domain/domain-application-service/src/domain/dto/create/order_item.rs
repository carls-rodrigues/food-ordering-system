use bigdecimal::BigDecimal;
use derive_builder::Builder;
use getset::Getters;
use uuid::Uuid;

#[derive(Debug, Builder, Getters)]
pub struct OrderItem {
    #[getset(get = "pub")]
    product_id: Uuid,
    #[getset(get = "pub")]
    quantity: i32,
    #[getset(get = "pub")]
    price: BigDecimal,
    #[getset(get = "pub")]
    subtotal: BigDecimal,
}
