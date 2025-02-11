use std::io;

fn main() {
    loop {
        let mut n = String::new();
        
        println!("To calculate the nth fibonacci, enter the value of n or enter 'done' to stop");
        let _ = io::stdin().read_line(&mut n);
        let n = n.trim();
        if n == "done" {
            return;
        } else {
            let n: i64 = match n.trim().parse() {
                Ok(n) => n,
                Err(_) =>{
                    println!("you entered :{}: .Enter a number", n);
                    continue;
            }
            };
            println!("the {}th fibonacci number is: {}", n, fibo(n));
    }
}
}

fn fibo(n: i64) -> i64 {

    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}
