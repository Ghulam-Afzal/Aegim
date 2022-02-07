use std::io; 


fn main() {
    println!("Start");

    let mut command = String::new(); 

    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read input"); 

    println!("> {}", command);
}
