use std::io::Error;

pub trait FormDataRectangle {
    fn new_rec() -> Self;
    fn collect_data(&mut self) -> Result<(), Error>;
}
