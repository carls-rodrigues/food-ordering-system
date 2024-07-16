use crate::domain::dto::message::PaymentResponse;

pub trait PaymentResponseMessageListener {
    fn payment_completed(&self, message: PaymentResponse) -> Result<(), String>;
    fn payment_cancelled(&self, message: PaymentResponse) -> Result<(), String>;
}
