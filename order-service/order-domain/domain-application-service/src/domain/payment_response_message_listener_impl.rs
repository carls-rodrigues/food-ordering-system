use crate::domain::dto::message::PaymentResponse;
use crate::domain::ports::input::message::listener::payment::PaymentResponseMessageListener;

#[derive(Debug, Default)]
pub struct PaymentResponseMessageListenerImpl {}

impl PaymentResponseMessageListenerImpl {}

impl PaymentResponseMessageListener for PaymentResponseMessageListenerImpl {
    fn payment_completed(&self, message: PaymentResponse) -> Result<(), String> {
        todo!()
    }

    fn payment_cancelled(&self, message: PaymentResponse) -> Result<(), String> {
        todo!()
    }
}
