use chrono::NaiveDateTime;
use common::domain::events::DomainEvent;
use getset::Getters;

use crate::domain::entity::Order;

#[derive(Debug, Getters)]
pub struct OrderCancelledEvent {
    #[getset(get = "pub")]
    order: Order,

    #[getset(get = "pub")]
    created_at: NaiveDateTime,
}

impl OrderCancelledEvent {
    pub fn new(order: Order) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            order,
            created_at: now,
        }
    }
}

impl DomainEvent<Order> for OrderCancelledEvent {}
