use std::io;
fn main(){
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
     

    //let mut age: i32 = 16;
    let mut year: i32 = 2024;
    
    //let mut range = 5;
    //while range >1{
        for _i in 1..10{
            println!("Mary will be {} years old in {}", age, year);
            age +=1;
            year +=1;
      //  }
        //range -= 1;

        
    }

    
}