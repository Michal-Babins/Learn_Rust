// Vectors are resizable arrays 

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    // re-assign value
    numbers[2] = 20;

    //Add on to vector
    numbers.push(6);

    //Pop off the last value
    numbers.pop();

    println!("{:?}", numbers);
    
    // Get single val
    println!("Single Value: {}", numbers[0]);

    //Get vec length
    println!("Vector Length: {}", numbers.len());

    // vectors are stack allocated
    println!("Vectore occupies {} bytes", mem::size_of_val(&numbers));

    //Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x)
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *=2;
    }

    println!("Numbers Vec: {:?}", numbers);
}