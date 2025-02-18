mod new;
use new::Pair;

fn main() {
    let pair_i32 = Pair::new(3, 4);
    let pair_i32 = pair_i32.swap();
    let pair_f64 = Pair::new(4.5, 10.8);
    let pair_f64 = pair_f64.swap();

    let pair_str = Pair::new("hello", 4);
    let pair_str = pair_str.swap();

    println!(
        "numbers:{:?}\n, float:{:?}\n, mixed{:?}\n",
        pair_i32, pair_f64, pair_str
    );
}
