pub trait BaseId<T> {
    fn new(id: T) -> Self;
    fn get_value(&self) -> &T;
}
