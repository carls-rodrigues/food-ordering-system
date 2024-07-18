use crate::domain::dto::track::{TrackOrderQuery, TrackOrderResponse};
use crate::domain::mapper::OrderDataMapper;
use crate::domain::ports::output::repository::OrderRepository;
use common::domain::exception::DomainException;
use common::domain::value_object::BaseId;
use order_domain_core::domain::exception::OrderNotFoundException;
use order_domain_core::domain::value_object::TrackingId;
use std::sync::Arc;

pub struct OrderTrackCommandHandler {
    order_data_mapper: Arc<OrderDataMapper>,
    order_repository: Arc<dyn OrderRepository>,
}
impl OrderTrackCommandHandler {
    pub fn new(
        order_data_mapper: OrderDataMapper,
        order_repository: impl OrderRepository + 'static,
    ) -> Self {
        Self {
            order_data_mapper: Arc::new(order_data_mapper),
            order_repository: Arc::new(order_repository),
        }
    }

    pub async fn track_order(
        &self,
        track_order_query: TrackOrderQuery,
    ) -> Result<TrackOrderResponse, DomainException<OrderNotFoundException>> {
        let order = self
            .order_repository
            .find_by_tracking_id(TrackingId::new(
                track_order_query.order_tracking_id().clone(),
            ))
            .await?;
        if order.is_none() {
            let message = format!(
                "Could not find order with tracking id: {}",
                track_order_query.order_tracking_id()
            );
            return Err(DomainException::NotFound(OrderNotFoundException::new(
                message, None,
            )));
        }
        Ok(self
            .order_data_mapper
            .order_to_create_order_response(&order.unwrap(), None))
    }
}
