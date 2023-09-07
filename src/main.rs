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

    // Floating point numbers demo
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
    println!("The value of 2x shadowed `x` is: {x}");
    print!("Type 2x shadowed `x` is: ");
    print_type_of(&x);

    println!("The value of `y` is: {y}");
    print!("Type of `y` is: ");
    print_type_of(&y);

    // Numeric Operations demo

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("The value of `sum` is: {sum}");
    println!("The value of `difference` is: {difference}");
    println!("The value of `product` is: {product}");
    println!("The value of `quotient` is: {quotient}");
    println!("The value of `truncated` is: {truncated}");
    print!("Type of `truncated` is: ");
    print_type_of(&truncated);
    println!("The value of `remainder` is: {remainder}");
}
