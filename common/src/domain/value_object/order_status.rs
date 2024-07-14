#[derive(Debug, PartialEq, Clone)]
pub enum OrderStatus {
    Pending,
    Paid,
    Approved,
    Cancelling,
    Cancelled,
}
