use common::domain::{entity::BaseEntity, value_object::Money};
use getset::Getters;
use common::domain::value_object::ProductId;

#[derive(Debug, Getters, PartialEq, Eq, Hash, Clone)]
pub struct Product {
    id: ProductId<uuid::Uuid>,
    #[getset(get = "pub")]
    name: Option<String>,
    #[getset(get = "pub")]
    price: Option<Money>,
}

impl Product {
    pub fn new(id: ProductId<uuid::Uuid>, name: Option<String>, price: Option<Money>) -> Self {
        Self { id, name, price }
    }
    pub fn update_with_confirmed_name_and_price(&mut self, name: &Option<String>, price: &Option<Money>) {
        self.name = name.clone();
        self.price = price.clone();
    }
}

impl BaseEntity<ProductId<uuid::Uuid>> for Product {
    fn get_id(&self) -> &ProductId<uuid::Uuid> {
        &self.id
    }

    fn set_id(&mut self, id: ProductId<uuid::Uuid>) {
        self.id = id;
    }
}
