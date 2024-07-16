use crate::domain::dto::{
    create::CreateOrderCommand,
    track::{TrackOrderQuery, TrackOrderResponse},
};

pub trait OrderApplicationService {
    fn create_order(
        &self,
        create_order_command: CreateOrderCommand,
    ) -> Result<CreateOrderCommand, String>;
    fn track_order(&self, track_order_quert: TrackOrderQuery)
        -> Result<TrackOrderResponse, String>;
}
