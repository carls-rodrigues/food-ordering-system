use super::base_id::BaseId;

#[derive(Debug, PartialEq, Clone)]
pub struct RestaurantId<T>
where
    T: From<uuid::Uuid> + Into<uuid::Uuid>,
{
    id: T,
}

impl<T> BaseId<T> for RestaurantId<T>
where
    T: From<uuid::Uuid> + Into<uuid::Uuid>,
{
    fn new(id: T) -> Self {
        Self { id }
    }

    fn get_value(&self) -> &T {
        &self.id
    }
}

#[cfg(test)]
mod restaurant_id_tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_restaurant_id() {
        let id = Uuid::now_v7();
        let restaurant_id = RestaurantId::new(id);
        assert_eq!(restaurant_id.get_value(), &id);
    }
}
