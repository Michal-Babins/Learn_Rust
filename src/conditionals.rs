//Conditional used to check condition of something and act on the result

pub fn run() {
    let age: u8 = 22;
    let check_id: bool = false;
    let knows_person_of_age = true;

    //if / else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, no drink for you.");
    } else {
        println!("Bartender: I'll need your ID")
    }

    // SHorthand if
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of Age: {}", is_of_age);
}