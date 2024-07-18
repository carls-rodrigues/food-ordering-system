use bigdecimal::BigDecimal;
use common::domain::value_object::ProductId;
use derive_builder::Builder;
use getset::Getters;
use uuid::Uuid;

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
