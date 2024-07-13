#[derive(Debug, PartialEq)]
pub enum OrderStatus {
    Pending,
    Paid,
    Approved,
    Cancelling,
    Cancelled,
}
