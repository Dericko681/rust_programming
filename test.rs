fn main(){
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", &s[0..5]);
    alt(&mut s);
    println!("{}", &s[6..10]);
}

fn change(s : &mut String){
    s.push_str(", world");
}

fn alt(s: &mut String){
    s.push_str("yes oooh");
}