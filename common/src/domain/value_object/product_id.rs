use super::base_id::BaseId;

pub struct ProductId<T>
where
    T: From<uuid::Uuid> + Into<uuid::Uuid>,
{
    id: T,
}

impl<T> BaseId<T> for ProductId<T>
where
    T: From<uuid::Uuid> + Into<uuid::Uuid>,
{
    fn new(id: T) -> Self {
        Self { id }
    }

    fn get_id(&self) -> &T {
        &self.id
    }
}

#[cfg(test)]
mod product_id_tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_product_id() {
        let id = Uuid::now_v7();
        let product_id = ProductId::new(id);
        assert_eq!(product_id.get_id(), &id);
    }
}
