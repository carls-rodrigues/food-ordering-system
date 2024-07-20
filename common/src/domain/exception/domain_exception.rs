use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum DomainException<T> {
    NotFound(T),
    DomainError(T),
}
