use std::{io, ops::Div, string};

use rand::seq::SliceRandom;
use rand::thread_rng;
use students::Students;

mod groups;
mod students;

fn main() {
    let mut student_vec: Vec<String> = Vec::new();
    let mut group_vec: Vec<String> = Vec::new();

    println!("Enter the names of students: Each on a new line. Type 'done' to end the list");

    loop {
        let mut names = String::new();
        io::stdin()
            .read_line(&mut names)
            .expect("failed to get value");
        // let names: String = names.trim();
        // trim().parse().unwrap();

        if names == "done" {
            break;
        } else {
            match names.parse::<String>() {
                Ok(valu) => student_vec.push(names),
                Err(_) => println!("invalid input. please enter a number\n"),
            }
        }
    }

    for i in &student_vec {
        let student_i = Students {
            name: i.to_string(),
        };
    }

    println!("Enter the names of Groups: Each on a new line. Type 'done' to end the list");

    loop {
        let mut groups = String::new();
        io::stdin()
            .read_line(&mut groups)
            .expect("failed to get value");
        // let names: String = names.trim();
        // trim().parse().unwrap();

        if groups == "done" {
            break;
        } else {
            match groups.parse::<String>() {
                Ok(valu) => group_vec.push(groups),
                Err(_) => println!("invalid input. please enter a number\n"),
            }
        }
    }
    let mut len_group = group_vec.len();
    let mut len_students = student_vec.len();

    let mut per_grp = len_students.div(len_group);

    let mut rng = thread_rng();
   
     for i in 0..per_grp {
         
    let mut selected_stud = student_vec
        .choose_multiple(&mut rng, per_grp)
        .cloned()
        .collect::<Vec<_>>();

        group_vec[i].push(selected_stud);
    }

    println!("{:?}", group_vec);
}
