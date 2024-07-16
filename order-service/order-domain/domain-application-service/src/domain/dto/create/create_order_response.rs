use common::domain::value_object::OrderStatus;
use derive_builder::Builder;
use getset::Getters;
use serde::Serialize;
use uuid::Uuid;
use order_domain_core::domain::value_object::TrackingId;

#[derive(Debug, Default, Builder, Getters, Serialize)]
pub struct CreateOrderResponse {
    #[serde(rename = "orderTrackingId")]
    order_tracking_id: Option<TrackingId<Uuid>>,
    #[serde(rename = "orderStatus")]
    order_status: OrderStatus,
    message: Option<String>,
}
