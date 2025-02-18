use std::ops::Div;

use crate::traitfile::Calculate;


pub struct Mean {
    pub vector: Vec<i32>,
}

impl Calculate for Mean {
    fn calc(vector: Vec<i32>) -> f32 {
        let vect: Vec<i32> = vector.clone();
        let length: i32 = vect.len() as i32;
        let sum: i32 = vect.iter().sum();
        sum.div(length) as f32
    }
}
