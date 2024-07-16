use bigdecimal::BigDecimal;
use derive_builder::Builder;
use getset::Getters;
use uuid::Uuid;
use crate::domain::dto::create::OrderItemDTO;
use super::order_address::OrderAddress;

#[derive(Debug, Default, Builder, Getters)]
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
