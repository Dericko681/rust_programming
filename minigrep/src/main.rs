use core::panic;
use std::{env::{self, args}, fs::{self, read}};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config=config::new(&args);
    impl config{
        fn new(args: &[String]) -> config{
        let query = args[1].clone();
        let file_path = args[2].clone();
        config { query, file_path }
    }
}

fn new(args: &[String]) -> config{
    if args.len() < 3 {
        panic!("insuficient arguements");
    }

}
}

struct config {
    query : String,
    file_path : String
}


fn parse_config(args: &[String]) -> config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    config{query, file_path}
}
