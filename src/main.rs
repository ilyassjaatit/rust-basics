use std::io;

fn main() {
    

    // Declarar variable mutable string
    let mut name = String::new();
    let mut age : String = String::new();

    println!("What is your name?");
    // set name by terminal 
    io::stdin().read_line(&mut name).unwrap();
    println!("How are old you?");
    // set age 
    io::stdin().read_line(&mut age).unwrap();

    let age_int :u8 = age.trim().parse().unwrap();

    println!("Hi! My name is {} and I am {} years old.", name, age_int)

}
