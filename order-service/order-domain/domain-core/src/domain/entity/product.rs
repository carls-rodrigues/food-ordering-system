use common::domain::{entity::BaseEntity, value_object::Money};
use getset::Getters;

#[derive(Debug, Getters)]
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
}

impl<ID: From<uuid::Uuid> + Into<uuid::Uuid> + PartialEq> BaseEntity<ID> for Product<ID> {
    fn get_id(&self) -> &ID {
        &self.id
    }

    fn set_id(&mut self, id: ID) {
        self.id = id;
    }
}
