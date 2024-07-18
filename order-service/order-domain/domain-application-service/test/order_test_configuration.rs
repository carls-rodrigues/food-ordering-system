#[cfg(test)]
use mockall::{automock, mock, predicate::*};

use order_domain_application_service::domain::ports::output::repository::message::publisher::payment::{OrderCancelledPaymentRequestMessagePublisher, OrderCreatedPaymentRequestMessagePublisher};
use order_domain_application_service::domain::ports::output::repository::message::publisher::restaurant_approval::OrderPaidRestaurantRequestMessagePublisher;
use order_domain_application_service::domain::ports::output::repository::OrderRepository;
use order_domain_core::domain::order_domain_service::OrderDomainService;
use order_domain_core::domain::order_domain_service_impl::OrderDomainServiceImpl;

#[derive(Default)]
pub struct OrderTestConfiguration;

impl OrderTestConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
    #[cfg_attr(test, automock)]
    pub fn order_created_payment_request_message_publisher(
        &self,
    ) -> Box<dyn OrderCreatedPaymentRequestMessagePublisher> {
        Box::new(OrderCreatedPaymentRequestMessagePublisher)
    }
    #[cfg_attr(test, automock)]
    pub fn order_cancelled_payment_request_message_publisher(
        &self,
    ) -> Box<dyn OrderCancelledPaymentRequestMessagePublisher> {
        Box::new(OrderCancelledPaymentRequestMessagePublisher)
    }
    #[cfg_attr(test, automock)]
    pub fn order_paid_payment_request_message_publisher(
        &self,
    ) -> Box<dyn OrderPaidRestaurantRequestMessagePublisher> {
        Box::new(OrderPaidRestaurantRequestMessagePublisher)
    }
    #[cfg_attr(test, automock)]
    pub fn order_repository(&self) -> Box<dyn OrderRepository> {
        Box::new(OrderRepository)
    }
    #[cfg_attr(test, automock)]
    pub fn customer_repository(&self) -> Box<dyn CustomerRepository> {
        Box::new(CustomerRepository)
    }
    #[cfg_attr(test, automock)]
    pub fn restaurant_repository(&self) -> Box<dyn RestaurantRepository> {
        Box::new(RestaurantRepository)
    }

    pub fn order_domain_service(&self) -> Box<dyn OrderDomainService> {
        Box::new(OrderDomainServiceImpl::new())
    }
}
