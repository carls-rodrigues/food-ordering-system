use common::domain::value_object::BaseId;

#[derive(Debug, PartialEq, Clone)]
pub struct TrackingId<T>
where
    T: From<uuid::Uuid> + Into<uuid::Uuid>,
{
    id: T,
}
impl<T> BaseId<T> for TrackingId<T>
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
mod tracking_id_tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_tracking_id() {
        let id = Uuid::now_v7();
        let tracking_id = TrackingId::new(id);
        assert_eq!(tracking_id.get_value(), &id);
    }
}
