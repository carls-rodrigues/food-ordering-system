pub trait BaseEntity<ID: PartialEq> {
    fn get_id(&self) -> &ID;
    fn set_id(&mut self, id: ID);
}
