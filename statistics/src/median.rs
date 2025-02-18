use std::ops::Div;

use crate::traitfile::Calculate;

pub struct Median {
    pub vector: Vec<i32>,
}

impl Calculate for Median {
    fn calc(mut vector: Vec<i32>) -> f32 {
        let mut ind = vector.len();
        vector.sort();

        if ind % 2 == 1 {
            vector[ind.div(2)] as f32
        }else{
            let mut var = ind.div(2);
            let mut ele = var+1;
            let mut result = (vector[var] +vector[ele]).div(2) as f32;
            result
        }
    }
}
