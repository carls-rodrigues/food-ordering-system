use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use common::domain::value_object::PaymentStatus;
use derive_builder::Builder;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Builder, Getters, Deserialize, Serialize)]
pub struct PaymentResponse {
    #[getset(get = "pub")]
    id: String,
    #[serde(rename = "sagaId")]
    #[getset(get = "pub")]
    saga_id: String,
    #[serde(rename = "orderId")]
    #[getset(get = "pub")]
    order_id: String,
    #[serde(rename = "paymentId")]
    #[getset(get = "pub")]
    payment_id: String,
    #[serde(rename = "customerId")]
    #[getset(get = "pub")]
    customer_id: String,
    #[getset(get = "pub")]
    price: BigDecimal,
    #[serde(rename = "createdAt")]
    #[getset(get = "pub")]
    created_at: NaiveDateTime,
    #[serde(rename = "paymentStatus")]
    #[getset(get = "pub")]
    payment_status: PaymentStatus,
    #[serde(rename = "failureMessages")]
    #[getset(get = "pub")]
    failure_messages: Vec<String>,
}
