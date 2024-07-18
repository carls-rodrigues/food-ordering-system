use crate::domain::dto::create::{CreateOrderCommand, CreateOrderResponse};
use crate::domain::dto::track::{TrackOrderQuery, TrackOrderResponse};
use crate::domain::order_create_command_handler::OrderCreateCommandHandler;
use crate::domain::order_track_command_handler::OrderTrackCommandHandler;
use crate::domain::ports::input::service::OrderApplicationService;
use async_trait::async_trait;
use common::domain::exception::DomainException;
use order_domain_core::domain::exception::{OrderDomainException, OrderNotFoundException};
use std::sync::Arc;

pub struct OrderApplicationServiceImpl {
    order_create_command_handler: Arc<OrderCreateCommandHandler>,
    order_track_command_handler: Arc<OrderTrackCommandHandler>,
}

impl OrderApplicationServiceImpl {
    pub fn new(
        order_create_command_handler: Arc<OrderCreateCommandHandler>,
        order_track_command_handler: Arc<OrderTrackCommandHandler>,
    ) -> Self {
        Self {
            order_create_command_handler,
            order_track_command_handler,
        }
    }
}

#[async_trait(?Send)]
impl OrderApplicationService for OrderApplicationServiceImpl {
    async fn create_order(
        &self,
        create_order_command: CreateOrderCommand,
    ) -> Result<CreateOrderResponse, DomainException<OrderDomainException>> {
        Ok(self
            .order_create_command_handler
            .create_order(create_order_command)
            .await?)
    }

    async fn track_order(
        &self,
        track_order_query: TrackOrderQuery,
    ) -> Result<TrackOrderResponse, DomainException<OrderNotFoundException>> {
        Ok(self
            .order_track_command_handler
            .track_order(track_order_query)
            .await?)
    }
}
