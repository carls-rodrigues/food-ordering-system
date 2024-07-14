use super::{
    entity::{Order, Restaurant},
    event::{OrderCancelledEvent, OrderCreatedEvent, OrderPaidEvent},
    exception::OrderDomainException,
};

pub trait OrderDomainService {
    fn validate_and_initiate_order(
        &self,
        order: Order,
        restaurant: Restaurant,
    ) -> Result<OrderCreatedEvent, OrderDomainException>;
    fn pay_order(&self, order: &mut Order) -> Result<OrderPaidEvent, OrderDomainException>;
    fn approve_order(&self, order: &mut Order) -> Result<(), OrderDomainException>;
    fn cancel_order_payment(
        &self,
        order: &mut Order,
        failure_messages: Vec<String>,
    ) -> Result<OrderCancelledEvent, OrderDomainException>;
    fn cancel_order(
        &self,
        order: &mut Order,
        failure_messages: Vec<String>,
    ) -> Result<(), OrderDomainException>;
}
