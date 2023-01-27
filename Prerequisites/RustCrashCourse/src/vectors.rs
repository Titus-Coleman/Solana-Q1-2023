// Vectors are resizable arrays

pub fn run() {
    let mut numbers: Vec<i32>= vec! [1,2,3,4,5];

    println!("{:?}", numbers);

    // single value
    println!("Single value: {}", numbers[0]);

    // reassign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // remove last value of the vector
    numbers.pop();

    // Get vector length
    println!("Vector length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..4];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2
    }

    println!("Numbers Vec: {:?}", numbers);
}