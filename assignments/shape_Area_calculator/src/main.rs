use rectangle::Rectangle;
use traits::form_data_rectangle::FormDataRectangle;


use crate::traits::area::Area;
mod circle;
mod rectangle;
mod traits;


    fn  main() {
            let mut rectangle = Rectangle::new_rec();
    loop {
       
        let result = rectangle.collect_data();
        if result.is_ok() {
            break;
        }
        eprintln!("{:?}", result);
    }
    println!("{:?} area: {}", rectangle, rectangle.area() )
    }