use common::domain::value_object::OrderStatus;
use derive_builder::Builder;
use getset::Getters;
use order_domain_core::domain::value_object::TrackingId;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Default, Builder, Getters, Serialize, PartialEq)]
pub struct CreateOrderResponse {
    #[serde(rename = "orderTrackingId")]
    #[getset(get = "pub")]
    order_tracking_id: Option<TrackingId<Uuid>>,
    #[serde(rename = "orderStatus")]
    #[getset(get = "pub")]
    order_status: OrderStatus,
    #[getset(get = "pub")]
    message: Option<String>,
}
