use std::{
    io::{self, Read},
    vec,
};
use mean::mean_value;

fn main() {
    let mut wan: Vec<i32> = Vec::new();

    loop {
        let mut v = String::new();
        io::stdin().read_line(&mut v).expect("msg");
        let v: i32 = v.trim().parse().expect("invalid");

        if v == -1 {
            break;
        } else {
            wan.push(v);
        
        }
        }


let mean = mean_value(wan);

println!("{mean}");

        // check if the values were correctly entered
//         println!("the value are {:#?}", wan);

//         // now we calculate the mean
//         let mut sum = 0 ; 
// let total ={
//     for i  in wan 
//     { sum = sum + i}
 }

//  println!("{total:?}");


    // }let lent = wan.len() as i32;
    // println!("the values of vector is: {:?}", wan);

    // let min = (wan.iter().sum::<i32>() / lent) as f64;

    // println!("the values of mean is: {min}");

mod mean;
mod test;