
fn main() {
    let s:[i32; 6] = [0, 1, 1, 1, 0, 1];
    let mut arr_l: Vec<i32> = Vec::new();
    let mut arr_c: Vec<i32> = Vec::new();
    for i in 0..s.len() {
        if s.len() == 1 {
            return;
        } else {
            let mut arr_l:Vec<i32> = arr_l.push(s[0]);
            let mut counter_0 = 0;
            let mut counter_1 = 0;

            for element in arr_l.iter() {
                if element == 0 {
                    counter_0 += 1;
                }
            }
            for element in s {
                if element == 1 {
                    counter_1 += 1;
                }
            }
            let sum = counter_0 + counter_1 + 1;
            arr_c.push(sum);
        }

    }
    println!("the value is {}", largest(arr_c));
}




fn largest(arr_c: Vec<i32>) -> i32{
    let  mut largest = arr_c[0];
    for i in arr_c{
        if i > largest{
            largest=i
        }
    }
    largest
}