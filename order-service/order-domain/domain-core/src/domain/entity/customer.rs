use common::domain::{
    entity::{AggregateRoot, BaseEntity},
    value_object::CustomerId,
};

pub struct Customer {
    id: CustomerId<uuid::Uuid>,
}

impl AggregateRoot<CustomerId<uuid::Uuid>> for Customer {}

impl BaseEntity<CustomerId<uuid::Uuid>> for Customer {
    fn get_id(&self) -> &CustomerId<uuid::Uuid> {
        &self.id
    }

    fn set_id(&mut self, id: CustomerId<uuid::Uuid>) {
        self.id = id;
    }
}
