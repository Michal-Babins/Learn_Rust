pub fn run() {
    //Print to console
    println!("Helo from print.rs file ");

    //Basic fomratting
    println!("{} is from {}", "Brad","Mass");

    //Positional Argument
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    //Named Arguments
    println!("{name} likes to play {activity}", name = "John", activity = "Baseball");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10,10,10);

    //Placeholder for debug trait 
    println!("{:?}", (12, true, "hello"));

    //Basic Math
    println!("10 + 10 = {}", 10 + 10);

}