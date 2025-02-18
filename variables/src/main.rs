use std::io;
mod test;
// mod factorial;
// use factorial::Factorial;
fn main() {
    //     let v = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v[2];
    // println!("The third element is {third}");
    // let third: Option<&i32> = v.get(2);
    // match third {
    // Some(third) => println!("The third element is {third}"),
    // None => println!("There is no third element."),
    // }

    let mut valu = String::new();
    let mut list1: Vec<i64> = Vec::new();
    list1.push(6);
    // let list=list.parse().expect("unable to get values");

    println!("enter the values to calculate the median and mode, type 0000 to finish\n");
    loop {
        io::stdin()
            .read_line(&mut valu)
            .expect("failed to get value");
        let valu = valu.trim();
        // trim().parse().unwrap();

        if valu == "done" {
            break;
        } else {
            match valu.parse::<i64>() {
                Ok(valu) => list1.push(valu),
                Err(_) => println!("invalid input. please enter a number\n"),
            }
        }
    }
    println!("values: {:?}", list1)
}
