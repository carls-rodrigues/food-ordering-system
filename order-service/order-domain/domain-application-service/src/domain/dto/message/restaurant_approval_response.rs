use chrono::NaiveDateTime;
use common::domain::value_object::OrderApprovalStatus;
use derive_builder::Builder;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Builder, Getters, Deserialize, Serialize)]
pub struct RestaurantApprovalResponse {
    #[getset(get = "pub")]
    id: String,
    #[serde(rename = "sagaId")]
    #[getset(get = "pub")]
    saga_id: String,
    #[serde(rename = "orderId")]
    #[getset(get = "pub")]
    order_id: String,
    #[serde(rename = "restaurantId")]
    #[getset(get = "pub")]
    restaurant_id: String,
    #[serde(rename = "createdAt")]
    #[getset(get = "pub")]
    created_at: NaiveDateTime,
    #[serde(rename = "orderApprovalStatus")]
    #[getset(get = "pub")]
    order_approval_status: OrderApprovalStatus,
    #[serde(rename = "failureMessages")]
    #[getset(get = "pub")]
    failure_messages: Vec<String>,
}
