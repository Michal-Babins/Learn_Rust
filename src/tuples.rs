// Tuples group together values 
// Max 12 elements


pub fn run() {
    let person: (&str, &str, i8) = ("Michal", "Mass", 23);

    println!("{} is from {} and is {}", person.0, person.1, person.2)

}