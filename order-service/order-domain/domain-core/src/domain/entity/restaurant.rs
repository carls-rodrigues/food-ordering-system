use super::Product;
use common::domain::{
    entity::{AggregateRoot, BaseEntity},
    value_object::{BaseId, RestaurantId},
};
use derive_builder::Builder;
use getset::Getters;

#[derive(Debug, Builder, Getters, Clone)]
pub struct Restaurant {
    id: RestaurantId<uuid::Uuid>,
    #[getset(get = "pub")]
    products: Vec<Product>,
    #[getset(get = "pub")]
    active: bool,
}

impl Restaurant {
    pub fn new(products: Vec<Product>, active: bool) -> Self {
        Restaurant::from(RestaurantId::new(uuid::Uuid::now_v7()), products, active)
    }
    pub fn from(id: RestaurantId<uuid::Uuid>, products: Vec<Product>, active: bool) -> Self {
        Self {
            id,
            products,
            active,
        }
    }
}

impl AggregateRoot<RestaurantId<uuid::Uuid>> for Restaurant {}

impl BaseEntity<RestaurantId<uuid::Uuid>> for Restaurant {
    fn get_id(&self) -> &RestaurantId<uuid::Uuid> {
        &self.id
    }

    fn set_id(&mut self, id: RestaurantId<uuid::Uuid>) {
        self.id = id;
    }
}
