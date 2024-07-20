use crate::domain::dto::message::RestaurantApprovalResponse;
use crate::domain::ports::input::message::listener::restaurant_approval::RestaurantApprovalResponseMessageListener;

pub struct RestaurantApprovalResponseMessageListenerImpl {}

impl RestaurantApprovalResponseMessageListener for RestaurantApprovalResponseMessageListenerImpl {
    fn order_approved(
        restaurant_approval_response: RestaurantApprovalResponse,
    ) -> Result<(), String> {
        todo!()
    }

    fn order_reject(
        restaurant_approval_response: RestaurantApprovalResponse,
    ) -> Result<(), String> {
        todo!()
    }
}
