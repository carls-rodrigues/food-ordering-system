use getset::Getters;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainException<T> {
    NotFound(T),
    DomainError(T),
}
