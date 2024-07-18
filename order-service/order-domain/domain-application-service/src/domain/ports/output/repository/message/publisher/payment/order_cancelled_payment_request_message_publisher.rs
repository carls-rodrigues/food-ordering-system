use common::domain::events::publisher::DomainEventPublisher;
use order_domain_core::domain::event::OrderCancelledEvent;

pub trait OrderCancelledPaymentRequestMessagePublisher:
    DomainEventPublisher<OrderCancelledEvent>
{
}
