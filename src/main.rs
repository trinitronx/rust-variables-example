use std::any::type_name;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>())
}
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
    print!("Type of constant `THREE_HOURS_IN_SECONDS` is: ");
    print_type_of(&THREE_HOURS_IN_SECONDS);

    let spaces = "   ";
    print!("Type of original variable `spaces` is: ");
    print_type_of(&spaces);
    let spaces = spaces.len();
    println!("Variable `spaces` is '{}'", spaces);
    print!("Type of shadowed variable `spaces` is: ");
    print_type_of(&spaces);
}