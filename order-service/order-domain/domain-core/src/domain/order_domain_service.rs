use super::{
    entity::{Order, Restaurant},
    event::{OrderCancelledEvent, OrderCreatedEvent, OrderPaidEvent},
    exception::OrderDomainException,
};
use common::domain::exception::DomainException;

pub trait OrderDomainService {
    fn validate_and_initiate_order(
        &self,
        order: Order,
        restaurant: Restaurant,
    ) -> Result<OrderCreatedEvent, DomainException<OrderDomainException>>;
    fn pay_order(
        &self,
        order: &mut Order,
    ) -> Result<OrderPaidEvent, DomainException<OrderDomainException>>;
    fn approve_order(&self, order: &mut Order)
        -> Result<(), DomainException<OrderDomainException>>;
    fn cancel_order_payment(
        &self,
        order: &mut Order,
        failure_messages: Vec<String>,
    ) -> Result<OrderCancelledEvent, DomainException<OrderDomainException>>;
    fn cancel_order(
        &self,
        order: &mut Order,
        failure_messages: Vec<String>,
    ) -> Result<(), DomainException<OrderDomainException>>;
}
