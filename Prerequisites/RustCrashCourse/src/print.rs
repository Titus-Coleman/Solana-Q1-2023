pub fn run(){
    // Print to consol
    println!("hello from the print.rs file");
    
    // Basic formatting
    println!("{} is from {}", "Titus", "Atlanta");

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Titus", "ATL", "code" );

    // Names Arguments
    println!("{name} likes to play {activity}",
        name = "Titus",
        activity = "Soccer");

    // Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10,10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}