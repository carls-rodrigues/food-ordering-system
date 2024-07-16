use common::domain::value_object::OrderStatus;
use derive_builder::Builder;
use getset::Getters;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Default, Builder, Getters, Serialize)]
pub struct CreateOrderResponse {
    #[serde(rename = "orderTrackingId")]
    order_tracking_id: Uuid,
    #[serde(rename = "orderStatus")]
    order_status: OrderStatus,
    message: String,
}
