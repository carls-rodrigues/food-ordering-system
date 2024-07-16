use common::domain::events::publisher::DomainEventPublisher;
use order_domain_core::domain::event::OrderPaidEvent;

pub trait OrderPaidRestaurantRequestMessagePublisher : DomainEventPublisher<OrderPaidEvent> {

}