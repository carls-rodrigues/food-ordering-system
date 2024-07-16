use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    #[default]
    Pending,
    Paid,
    Approved,
    Cancelling,
    Cancelled,
}
