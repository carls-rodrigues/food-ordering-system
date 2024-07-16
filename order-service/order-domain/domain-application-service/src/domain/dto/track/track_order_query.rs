use derive_builder::Builder;
use getset::Getters;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Default, Debug, Builder, Getters, Deserialize)]
pub struct TrackOrderQuery {
    order_tracking_id: Uuid,
}
