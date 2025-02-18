// // program takes the number ofnumbers
// // program takes operations
// // program calculates arithmethics
// // program print's output

use std::io;

// // fn calculate() {
// //     loop {}
// // }

fn operations(num1: i64, num2: i64, operator: char) -> i64 {
    // let mut result1:i64;
    if operator == '+' {
        return num1 + num2;
    } else if operator == '-' {
        return num1 - num2;
    } else if operator == '*' {
        return num1 * num2;
    } else if operator == '/' {
        return num1 / num2;
    } else {
        1
    }
}

fn test() {
    println!("enter the first number\n");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("invalid input");
    let num1: i64 = num1.trim().parse().unwrap();

    println!("enter the second number\n");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("invalid input");
    let num2: i64 = num2.trim().parse().unwrap();

    println!("enter the operator\n");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator);
    let operator: char = operator.trim().parse().unwrap();
}

fn main() {
     test();
    operations(num1, num2, operator);
   
    let mut result1: i64 = operations(num1, num2, operator);
    println!("{result1}");
    //     let mut operator = String::new();
    //     io::stdin().read_line(&mut operator);
    //     let operator: char = operator.trim().parse().unwrap();
    loop {
        if operator == '=' {
            break;
        } else {
            test();
        }
    }
}
