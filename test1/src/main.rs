// use std::io;

// fn main() {
//     let mut string1 = String::new();
//     let mut string2 = String::new();
//     println!("Enter the first string");
//     io::stdin()
//         .read_line(&mut string1)
//         .expect("Enter a valid string");
//     // let string1 = match string1 {
//     //     string1 => string1,
//     //     _ => {
//     //         println!("Enter a valid String");
//     //     }
//     // }
//     println!("Enter the second string");
//     io::stdin()
//         .read_line(&mut string2)
//         .expect("Enter a valid string");
//     let res = longer(string1.clone(), string2.clone());
//     println!(
//         "The  longer string of '{:?}' and '{:?}'is '{:?}'",
//         string1, string2, res
//     );
// }

// fn longer(string1: String, string2: String) -> String {
//     let mut longer = String::new();
//     if string1.len() > string2.len() {
//         let longer = string1;
//         longer
//     } else {
//         let longer = string2;
//         longer
//     }
// }

fn add(num1: u32, num2: u32) -> u32 {
    num1 + num2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
    #[test]
    fn pan() {
        panic!("Learning automated testing");
    }
}
