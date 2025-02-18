use std::{env, io};

use mean::Mean;

use median::Median;
use traitfile::Calculate;
//use std::io;

mod mean;
mod mode;
mod median;
mod traitfile;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut vector: Vec<i32> = Vec::new();
    // let mut mean = Mean{ vector: vector };
    let mut median = Median{vector};

    // io::stdin().read_line(&args);

    if args.len() > 1 {
        for arg in args.iter().skip(1) {
            match arg.trim().parse::<i32>() {
                Ok(num) => median.vector.push(num),
                Err(_) => println!("Error: unable to parse argument"),
            }
        }
    } else {
        println!("Error: no arguments provided");
    }
    let ans = <Mean as Calculate>::calc(median.vector);
 println!("the mean is :{} ", ans);
//  println!("the median is {}", <Median as Calculate>::calc(median.vector));
    
}