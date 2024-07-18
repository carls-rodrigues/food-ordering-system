use common::domain::value_object::OrderStatus;
use derive_builder::Builder;
use getset::Getters;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use order_domain_core::domain::value_object::TrackingId;

#[derive(Default, Debug, Builder, Getters, Deserialize, Serialize)]
pub struct TrackOrderResponse {
    #[serde(rename = "orderTrackingId")]
    order_tracking_id: TrackingId<Uuid>,
    #[serde(rename = "orderStatus")]
    order_status: OrderStatus,
    #[serde(rename = "failureMessage")]
    failure_message: Vec<String>,
}
