use common::domain::{
    entity::{AggregateRoot, BaseEntity},
    value_object::{BaseId, RestaurantId},
};
use getset::Getters;

use super::Product;

#[derive(Debug, Getters)]
pub struct Restaurant {
    id: RestaurantId<uuid::Uuid>,
    #[getset(get = "pub")]
    products: Vec<Product<uuid::Uuid>>,
    #[getset(get = "pub")]
    active: bool,
}

impl Restaurant {
    pub fn new(products: Vec<Product<uuid::Uuid>>, active: bool) -> Self {
        Restaurant::from(RestaurantId::new(uuid::Uuid::now_v7()), products, active)
    }
    pub fn from(
        id: RestaurantId<uuid::Uuid>,
        products: Vec<Product<uuid::Uuid>>,
        active: bool,
    ) -> Self {
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
