use bigdecimal::BigDecimal;
use derive_builder::Builder;
use getset::Getters;
use order_domain_core::domain::entity::OrderItem;
use uuid::Uuid;

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
    order_items: Vec<OrderItem>,
    #[getset(get = "pub")]
    order_address: OrderAddress,
}
