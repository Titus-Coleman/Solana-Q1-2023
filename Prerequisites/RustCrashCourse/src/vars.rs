pub fn run(){
    let name = "Titus";
    let mut age = 37;

    println!("My name is {} and I am {}", name, age);

    // Define Constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign mutiple vars
    let (my_name, my_age ) = ("Titus", 37);
    println!("My name is {} I am {}", my_name, my_age);
}