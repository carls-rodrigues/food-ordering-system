use crate::domain::events::DomainEvent;

pub trait DomainEventPublisher<T>: DomainEvent<T> {
    fn publish(&self, event: T);
}
