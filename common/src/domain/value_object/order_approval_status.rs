use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OrderApprovalStatus {
    #[default]
    Pending,
    Approved,
    Rejected,
}
