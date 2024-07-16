use async_trait::async_trait;
use order_domain_core::domain::{entity::Order, value_object::TrackingId};
use uuid::Uuid;
use order_domain_core::domain::exception::OrderDomainException;

#[async_trait]
pub trait OrderRepository {
    async fn save(&self, order: Order) -> Result<Order, OrderDomainException>;
    async fn find_by_tracking_id(
        &self,
        tracking_id: TrackingId<Uuid>,
    ) -> Result<Option<Order>, OrderDomainException>;
}
