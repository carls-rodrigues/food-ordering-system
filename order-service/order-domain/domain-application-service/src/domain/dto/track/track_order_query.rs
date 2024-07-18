use derive_builder::Builder;
use getset::Getters;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Default, Debug, Builder, Getters, Deserialize)]
pub struct TrackOrderQuery {
    #[getset(get = "pub")]
    order_tracking_id: Uuid,
}
