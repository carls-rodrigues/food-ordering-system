use crate::domain::dto::track::{TrackOrderQuery, TrackOrderResponse};
use order_domain_core::domain::exception::OrderDomainException;

pub struct OrderTrackCommandHandler {}
impl OrderTrackCommandHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn track_order(
        &self,
        track_order_query: TrackOrderQuery,
    ) -> Result<TrackOrderResponse, OrderDomainException> {
        todo!()
    }
}
