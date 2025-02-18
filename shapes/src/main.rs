use std::env::args;
mod shape;
mod area_trait;
use shape::{Circle, Rectangle, Triangle};
use crate::area_trait::Area;

fn main() {
  let cir1=Circle::new(2.0);{
    //r:2,
  };

  let rect1=Rectangle::new(4,6);{
    // width: 6,
    // length: 4,
  };

  let tria1=Triangle::new(8,4);{
   
  };

  println!("circle: {:?}\n rectangle: {:?}\n, triangle: {:?}\n", cir1, rect1, tria1);
  }