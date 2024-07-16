use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use common::domain::value_object::PaymentStatus;
use derive_builder::Builder;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Builder, Getters, Deserialize, Serialize)]
pub struct PaymentResponse {
    id: String,
    #[serde(rename = "sagaId")]
    saga_id: String,
    #[serde(rename = "orderId")]
    order_id: String,
    #[serde(rename = "paymentId")]
    payment_id: String,
    #[serde(rename = "customerId")]
    customer_id: String,
    price: BigDecimal,
    #[serde(rename = "createdAt")]
    created_at: NaiveDateTime,
    #[serde(rename = "paymentStatus")]
    payment_status: PaymentStatus,
    #[serde(rename = "failureMessages")]
    failure_messages: Vec<String>,
}
