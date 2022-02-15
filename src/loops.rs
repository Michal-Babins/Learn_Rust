//Loops - uusee to iterate until a condition is met and

pub fn run(){
    let mut count = 0;

    //infinit loop
    // loop{
    //     count += 1;
    //     println!{"Number: {}", count};

    //     if count ==20 {
    //         break;
    //     }
    // }

    //While loop (FizzBuzz)
    // while count <= 100 {
    //     if count % 15 == 0 {
    //         println!{"FizzBuzz: {}", count};
    //     } else if count % 3 == 0 {
    //         println!("fizz");
    //     } else if count % 5 == 0 {
    //         println!("Buzz");
    //     } else {
    //         println!("Number: {}", count);
    //     }

    //     //count increment 
    //     count += 1;
    // }

    //For range
    for x in 0..100 {
        if x % 15 == 0 {
            println!{"FizzBuzz: {}", x};
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("Number: {}", x);
        }
    }
}
