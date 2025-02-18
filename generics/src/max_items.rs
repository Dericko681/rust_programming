pub struct MaxItem ;
impl MaxItem{
    pub fn find_max<T: PartialOrd + Copy> (slice: &[T]) -> &T {
        let mut max_value = &slice[0];   
        for item in slice {
            if item > max_value{
              max_value = item;
            }
        }
    max_value
    }
    
}