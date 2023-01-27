pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", numbers);

    // single value
    println!("Single value: {}", numbers[0]);

    // reassign value
    numbers[2] = 20;

    // Get array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}