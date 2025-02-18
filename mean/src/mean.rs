use std::ops::Div;

pub fn mean_value(vector:Vec<i32>) -> f64 {

let length:f64 = vector.len() as f64;

// let mut sum:f64 = 0_f64;
let total:f64 = {
    let mut sum = 0;
    for i in vector {
    sum = sum + i;
}
sum as f64
};

let mean = total.div(length);
mean

}