use common::domain::{entity::BaseEntity, value_object::Money};
use getset::Getters;

#[derive(Debug, Getters, PartialEq, Eq, Hash, Clone)]
pub struct Product<ID>
where
    ID: From<uuid::Uuid> + Into<uuid::Uuid> + PartialEq,
{
    id: ID,
    #[getset(get = "pub")]
    name: String,
    #[getset(get = "pub")]
    price: Money,
}

impl<ID: From<uuid::Uuid> + Into<uuid::Uuid> + PartialEq> Product<ID> {
    pub fn new(id: ID, name: String, price: Money) -> Self {
        Self { id, name, price }
    }
    pub fn update_with_confirmed_name_and_price(&mut self, name: &String, price: &Money) {
        self.name = name.clone();
        self.price = price.clone();
    }
}

impl<ID: From<uuid::Uuid> + Into<uuid::Uuid> + PartialEq> BaseEntity<ID> for Product<ID> {
    fn get_id(&self) -> &ID {
        &self.id
    }

    fn set_id(&mut self, id: ID) {
        self.id = id;
    }
}
