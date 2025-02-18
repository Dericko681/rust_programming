fn restore_ip_addr(s: String) -> Vec<String> {
    fn is_valid(sub: &str) -> bool{
        if sub.len()>1 && sub.starts_with('0'){
            return false;
        }
        let num: u16 = sub.trim().parse().unwrap();
        num <= 255
    }
    fn backtrack(s: &str, path: &mut Vec<String>, res: &mut Vec<String>){
        if path.len()==4 {
            if s.is_empty(){
                res.push(path.join("."));
            }
            return;
        }
        for i in 1..=3 {
            if i > s.len(){
            break;
        }
        let sub=&s[..i];
        if is_valid(sub){
            path.push(sub.to_string());
            backtrack(&s[i..], path, res);
            path.pop();
        }    }
}
let mut res = Vec::new();
backtrack(&s, &mut 
Vec::new(), &mut res);
res
}

fn main(){
    let s: String =String::from("25525511135");
    let res = restore_ip_addr(s);
println!("{:?}", res);
}