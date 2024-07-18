use async_trait::async_trait;
use crate::domain::events::DomainEvent;

#[async_trait]
pub trait DomainEventPublisher<T>: DomainEvent<T> {
    async fn publish(&self, event: &T);
}
