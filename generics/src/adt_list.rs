/// Abstract data types
pub trait ADTList<T> {
    /// Innitialize the ADT list
    fn new() -> Self;

    ///  Return an element from the list at any given position.
    fn get(&self, pos: u32) -> Option<T>;

    ///  Insert an element at end of the list.
    fn insert(&mut self, data: T);

    ///  Remove the first occurrence of any element from a non-empty list.
    fn remove(&mut self, pos: u32, data: T);
/// Remove last element of the list
    fn pop(&mut self);


    ///  Remove the element at a specified location from a non-empty list.
    fn remove_at(&self, pos: u32) -> Option<T>;

    ///  Replace an element at any position with another element.
    fn replace(&self, data: T, pos: u32) -> Self;

    ///  Return the number of elements in the list.
    fn size(&self) -> u32;

    ///  Return true if the list is empty; otherwise, return false.
    fn isEmpty(&self) -> bool;
}
