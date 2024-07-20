use crate::domain::dto::create::CreateOrderResponse;
use crate::domain::dto::{
    create::CreateOrderCommand,
    track::{TrackOrderQuery, TrackOrderResponse},
};
use async_trait::async_trait;
use common::domain::exception::DomainException;
use order_domain_core::domain::exception::{OrderDomainException, OrderNotFoundException};

#[async_trait(?Send)]
pub trait OrderApplicationService {
    async fn create_order(
        &self,
        create_order_command: CreateOrderCommand,
    ) -> Result<CreateOrderResponse, DomainException<OrderDomainException>>;
    async fn track_order(
        &self,
        track_order_query: TrackOrderQuery,
    ) -> Result<TrackOrderResponse, DomainException<OrderNotFoundException>>;
}
