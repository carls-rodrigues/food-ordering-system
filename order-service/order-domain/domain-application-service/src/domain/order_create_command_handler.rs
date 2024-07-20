use crate::domain::dto::create::{CreateOrderCommand, CreateOrderResponse};
use crate::domain::mapper::OrderDataMapper;
use crate::domain::ports::output::repository::message::publisher::payment::OrderCreatedPaymentRequestMessagePublisher;
use crate::domain::OrderCreateHelper;
use common::domain::entity::BaseEntity;
use common::domain::exception::DomainException;
use common::domain::value_object::BaseId;
use order_domain_core::domain::exception::OrderDomainException;
use std::sync::Arc;

pub struct OrderCreateCommandHandler {
    order_create_helper: Arc<OrderCreateHelper>,
    order_data_mapper: Arc<OrderDataMapper>,
    order_created_payment_request_message_publisher:
        Arc<dyn OrderCreatedPaymentRequestMessagePublisher>,
}

impl OrderCreateCommandHandler {
    pub fn new(
        order_create_helper: Arc<OrderCreateHelper>,
        order_data_mapper: Arc<OrderDataMapper>,
        order_created_payment_request_message_publisher: impl OrderCreatedPaymentRequestMessagePublisher
            + 'static,
    ) -> Self {
        Self {
            order_create_helper,
            order_data_mapper,
            order_created_payment_request_message_publisher: Arc::new(
                order_created_payment_request_message_publisher,
            ),
        }
    }

    pub async fn create_order(
        &self,
        create_order_command: CreateOrderCommand,
    ) -> Result<CreateOrderResponse, DomainException<OrderDomainException>> {
        let order_created_event = self
            .order_create_helper
            .persist_order(create_order_command)
            .await?;
        tracing::info!(
            "Order created with order_id: {}",
            order_created_event.order().get_id().get_value()
        );
        self.order_created_payment_request_message_publisher
            .publish(&order_created_event)
            .await;
        Ok(self
            .order_data_mapper
            .order_to_create_order_response(order_created_event.order(), None))
    }
}
