use std::io;
fn main() {
    let mut input = String ::new();
    let string_1 = String::from("Hi");
    println!("What's your name:");
    io::stdin().read_line(&mut input).expect("failed to read line");
    let name=input;
    //println!("{} {}",string_1 ,input); //this line will cause an error bcuz of the ownership
    println!("{} {}",string_1 ,name);
}