fn largest(arr_c: Vec<i32>) -> i32{
    let  mut largest = arr_c[0];
    for i in arr_c{
        if i > largest{
            largest=i
        }
    }
    largest
}

fn main(){
    let arr_c: Vec<i32> = vec![1, 3, 10, 4, 5];
    println!("the value is {}", largest(arr_c));
}