use common::domain::{
    entity::{AggregateRoot, BaseEntity},
    value_object::{CustomerId, Money, OrderStatus, RestaurantId},
};
use getset::Getters;

use crate::domain::value_object::{OrderItemId, StreetAddress, TrackingId};

use super::OrderItem;

pub struct OrderData<ID>
where
    ID: PartialEq,
{
    pub id: ID,
    pub customer_id: CustomerId<uuid::Uuid>,
    pub restaurant_id: RestaurantId<uuid::Uuid>,
    pub street_address: StreetAddress,
    pub price: Money,
    pub items: Vec<OrderItem<OrderItemId<uuid::Uuid>>>,
    pub tracking_id: TrackingId<uuid::Uuid>,
    pub order_status: OrderStatus,
    pub failure_messages: Vec<String>,
}

#[derive(Debug, Getters)]
pub struct Order<ID>
where
    ID: PartialEq,
{
    id: ID,
    #[getset(get = "pub")]
    customer_id: CustomerId<uuid::Uuid>,
    #[getset(get = "pub")]
    restaurant_id: RestaurantId<uuid::Uuid>,
    #[getset(get = "pub")]
    street_address: StreetAddress,
    #[getset(get = "pub")]
    price: Money,
    #[getset(get = "pub")]
    items: Vec<OrderItem<OrderItemId<uuid::Uuid>>>,
    #[getset(get = "pub")]
    tracking_id: TrackingId<uuid::Uuid>,
    #[getset(get = "pub")]
    order_status: OrderStatus,
    #[getset(get = "pub")]
    failure_messages: Vec<String>,
}

impl<ID: PartialEq> Order<ID> {
    pub fn new(order: OrderData<ID>) -> Self {
        Self {
            id: order.id,
            customer_id: order.customer_id,
            restaurant_id: order.restaurant_id,
            street_address: order.street_address,
            price: order.price,
            items: order.items,
            tracking_id: order.tracking_id,
            order_status: order.order_status,
            failure_messages: order.failure_messages,
        }
    }
}

impl<ID: PartialEq> AggregateRoot<ID> for Order<ID> {}

impl<ID: PartialEq> BaseEntity<ID> for Order<ID> {
    fn get_id(&self) -> &ID {
        &self.id
    }

    fn set_id(&mut self, id: ID) {
        self.id = id;
    }
}
