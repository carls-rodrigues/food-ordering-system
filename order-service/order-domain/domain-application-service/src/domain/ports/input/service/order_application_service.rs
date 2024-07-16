use async_trait::async_trait;
use order_domain_core::domain::exception::OrderDomainException;
use crate::domain::dto::{
    create::CreateOrderCommand,
    track::{TrackOrderQuery, TrackOrderResponse},
};
use crate::domain::dto::create::CreateOrderResponse;

#[async_trait(?Send)]
pub trait OrderApplicationService {
    async fn create_order(
        &self,
        create_order_command: CreateOrderCommand,
    ) -> Result<CreateOrderResponse, OrderDomainException>;
    async fn track_order(&self, track_order_query: TrackOrderQuery)
        -> Result<TrackOrderResponse, OrderDomainException>;
}
