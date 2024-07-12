pub trait BaseEntity<ID: PartialEq> {
    fn new(id: ID) -> Self;
    fn get_id(&self) -> &ID;
    fn set_id(&mut self, id: ID);
}
