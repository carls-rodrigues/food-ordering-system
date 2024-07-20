use async_trait::async_trait;
use common::domain::exception::DomainException;
use order_domain_core::domain::exception::{OrderDomainException, OrderNotFoundException};
use order_domain_core::domain::{entity::Order, value_object::TrackingId};
use uuid::Uuid;

#[async_trait]
pub trait OrderRepository {
    async fn save(&self, order: Order) -> Result<Order, DomainException<OrderDomainException>>;
    async fn find_by_tracking_id(
        &self,
        tracking_id: TrackingId<Uuid>,
    ) -> Result<Option<Order>, DomainException<OrderNotFoundException>>;
}
