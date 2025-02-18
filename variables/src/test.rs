fn main() {
    let mut name = String::new();
    let mut othe: Vec<String> = Vec::new();

    loop {
        if name == "@" {
            break;
        } else {
            othe.push(name.clone())
        }
        println!("This is the output:{othe:?}");
    }
}
