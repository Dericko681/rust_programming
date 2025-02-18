// rectangle.rs
use std::io;
use std::io::{Error, ErrorKind};
use crate::traits::area::Area;
use crate::traits::form_data_rectangle::FormDataRectangle;

#[derive(Debug)]
pub struct Rectangle {
    width: f64,
    height: f64,
}

// impl Rectangle {
//     pub fn from_rec(width: f64, height: f64) -> Self {
//         Self { width, height }
//     }

//     pub fn set_width(&mut self, width: f64) {
//         self.width = width;
//     }

//     pub fn get_width(&self) -> f64 {
//         self.width
//     }

//     pub fn set_height(&mut self, height: f64) {
//         self.height = height;
//     }

//     pub fn get_height(&self) -> f64 {
//         self.height
//     }
// }

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl FormDataRectangle for Rectangle {
   
     fn collect_data(&mut self) -> Result<(), Error> {
        println!("Enter the rectangle width: ");
        let mut width = String::new();
        io::stdin().read_line(&mut width)?;

let width = match width.trim().parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Invalid input for width. Please enter a valid number.",
                ))
            }
        };
//      5       
        println!("Enter the rectangle height: ");
        let mut height = String::new();
        io::stdin().read_line(&mut height)?;

        let height = match height.trim().parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Invalid input for height. Please enter a valid number.",
                ))
            }
        };
        self.height = height;
        self.width = width;
        Ok(())
    }
    
    fn new_rec() -> Self {
        Self { height: 0.0, width: 0.0 }
}
}
