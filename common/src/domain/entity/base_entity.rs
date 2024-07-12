// #[derive(Debug, Clone, PartialEq)]
pub trait BaseEntity<ID: PartialEq> {
    fn new(id: ID) -> Self;
    fn get_id(&self) -> &ID;
    fn set_id(&mut self, id: ID);
    // pub id: ID,
}

// impl<ID> BaseEntity<ID> {
//     pub fn new(id: ID) -> Self {
//         Self { id }
//     }
//     pub fn get_id(&self) -> &ID {
//         &self.id
//     }
//     pub fn set_d(&mut self, id: ID) {
//         self.id = id;
//     }
// }
