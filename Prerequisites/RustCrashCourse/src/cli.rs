use std::env;

///Basics of a CLI

pub fn run(){
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Titus";
    let status = "Ready";

    // println!("Command: {:?}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not a valid command");
    }
}