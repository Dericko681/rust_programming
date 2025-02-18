use std::io::Error;

pub trait FormDataCircle {
    fn new_cir() -> Self;
    fn collect_data(&mut self) -> Result<(), Error>;
}
