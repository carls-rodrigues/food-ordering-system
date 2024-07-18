use crate::domain::events::DomainEvent;
use async_trait::async_trait;

#[async_trait]
pub trait DomainEventPublisher<T>: DomainEvent<T> {
    async fn publish(&self, event: &T);
}
