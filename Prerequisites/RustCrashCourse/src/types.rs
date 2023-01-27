pub fn run(){


    //inferred 
    let x =1;
    let y =2.5; 

    //explict 
    let z: i64 = 3903093;

    // Find max size;
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i32: {}", std::i64::MAX);

    // Booleans
    let is_active = true;
    let is_greater: bool = 10>5;

    //Char
    let a1: char = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x,y,z,is_active, is_greater, a1, face));
}