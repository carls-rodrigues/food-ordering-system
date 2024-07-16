use async_trait::async_trait;
use common::domain::events::publisher::DomainEventPublisher;
use order_domain_core::domain::event::OrderCreatedEvent;

#[async_trait]
pub trait OrderCreatedPaymentRequestMessagePublisher:
    DomainEventPublisher<OrderCreatedEvent>
{
}
