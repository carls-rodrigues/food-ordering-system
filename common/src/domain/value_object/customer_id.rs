use super::base_id::BaseId;

pub struct CustomerId<T>
where
    T: From<uuid::Uuid> + Into<uuid::Uuid>,
{
    id: T,
}

impl<T> BaseId<T> for CustomerId<T>
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
mod customer_id_tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_customer_id() {
        let id = Uuid::now_v7();
        let customer_id = CustomerId::new(id);
        assert_eq!(customer_id.get_id(), &id);
    }
}
