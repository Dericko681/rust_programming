use std::env;

fn main() {
    let cur_directory = env::current_dir().unwrap();
    let num_cores = num_cpus::get();
    let os_system = std::env::consts::OS;
    println!("the current directory is {}, and the  number of cpu cores is {},
    and the  os type is {}", cur_directory.to_str().unwrap(), num_cores, os_system);
}
