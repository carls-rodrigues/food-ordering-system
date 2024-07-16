use chrono::NaiveDateTime;
use common::domain::value_object::OrderApprovalStatus;
use derive_builder::Builder;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Builder, Getters, Deserialize, Serialize)]
pub struct RestaurantApprovalResponse {
    id: String,
    #[serde(rename = "sagaId")]
    saga_id: String,
    #[serde(rename = "orderId")]
    order_id: String,
    #[serde(rename = "restaurantId")]
    restaurant_id: String,
    #[serde(rename = "createdAt")]
    created_at: NaiveDateTime,
    #[serde(rename = "orderApprovalStatus")]
    order_approval_status: OrderApprovalStatus,
    #[serde(rename = "failureMessages")]
    failure_messages: Vec<String>,
}
