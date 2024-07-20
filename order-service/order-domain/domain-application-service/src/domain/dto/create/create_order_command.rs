use super::order_address::OrderAddress;
use crate::domain::dto::create::OrderItemDTO;
use bigdecimal::BigDecimal;
use derive_builder::Builder;
use getset::Getters;
use uuid::Uuid;

#[derive(Debug, Default, Builder, Getters, PartialEq)]
pub struct CreateOrderCommand {
    #[getset(get = "pub")]
    customer_id: Uuid,
    #[getset(get = "pub")]
    restaurant_id: Uuid,
    #[getset(get = "pub")]
    price: BigDecimal,
    #[getset(get = "pub")]
    order_items: Vec<OrderItemDTO>,
    #[getset(get = "pub")]
    order_address: OrderAddress,
}
