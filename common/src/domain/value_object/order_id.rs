use super::base_id::BaseId;

#[derive(Debug, PartialEq, Clone)]
pub struct OrderId<T>
where
    T: From<uuid::Uuid> + Into<uuid::Uuid>,
{
    id: T,
}

impl<T> BaseId<T> for OrderId<T>
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
mod order_id_tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_order_id() {
        let id = Uuid::now_v7();
        let order_id = OrderId::new(id);
        assert_eq!(order_id.get_id(), &id);
    }
}
