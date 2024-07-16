use bigdecimal::BigDecimal;
use derive_builder::Builder;
use getset::Getters;
use uuid::Uuid;
use common::domain::value_object::ProductId;

#[derive(Debug, Builder, Getters, Clone)]
pub struct OrderItemDTO {
    #[getset(get = "pub")]
    product_id: ProductId<Uuid>,
    #[getset(get = "pub")]
    quantity: i32,
    #[getset(get = "pub")]
    price: BigDecimal,
    #[getset(get = "pub")]
    subtotal: BigDecimal,
}
