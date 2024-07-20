use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DomainException<T> {
    NotFound(T),
    DomainError(T),
}
