pub trait AccessMut{
    fn get(&self) -> &str;
    fn set(&self) -> &str;
}