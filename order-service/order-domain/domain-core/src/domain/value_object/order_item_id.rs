use common::domain::value_object::BaseId;

#[derive(Debug, PartialEq)]
pub struct OrderItemId<T> {
    id: T,
}
impl<T> BaseId<T> for OrderItemId<T>
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
mod order_item_id_tests {
    use super::*;

    #[test]
    fn test_order_item_id() {
        let id = uuid::Uuid::now_v7();
        let order_item_id = OrderItemId::new(id);
        assert_eq!(order_item_id.get_id(), &id);
    }
}
