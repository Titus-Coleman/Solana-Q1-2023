// Enums are types that have a few defined values

enum Movement{
    //Varients
    Up,
    Down,
    Left,
    Right
}


fn move_avatar(m: Movement){
    // Perform action based on enum varient

    match m {
        Movement::Up => println!("Avatar moving Up"),
        Movement::Down => println!("Avatar moving Down"),
        Movement::Left => println!("Avatar moving Left"),
        Movement::Right => println!("Avatar moving Right"),

    }
}

pub fn run(){
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Down;
    let avatar3 = Movement::Up;
    let avatar4 = Movement::Right;
    
    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
