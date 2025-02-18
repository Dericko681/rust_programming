use std::f64::consts::PI;
use crate::area_trait::Area;

#[derive(Debug)]
pub  struct Circle{
      r: f64,
}
impl Circle{
    pub fn new(r:f64) -> Self {
        Self{r}
    }
}

impl Area for Circle{
    fn calculate(&self) -> f64 {
        self.r *PI
    }
}

#[derive(Debug)]
 pub struct Rectangle {
     width: i64,
     length: i64,
}

impl Rectangle{
    pub fn new(width: i64, length:i64) -> Self{
        Self{width, length}
    }
}

#[derive(Debug)]
 pub struct Triangle{
    height: i32,
     base: i32,
}

impl Triangle {
    pub fn new(base: i32, height:i32) -> Self {
        Self{base, height}
    }
}