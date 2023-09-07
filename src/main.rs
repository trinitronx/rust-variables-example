use std::any::type_name;
extern crate atty;

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

    // Boolean types
    let t = true;

    let f: bool = false; // with explicit type annotation
    println!("The value of `t` is: {t}");

    print!("Type of `t` is: ");
    print_type_of(&t);
    println!("The value of `f` is: {f}");

    // Char type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("The value of `c` is: {c}");
    print!("Type of `c` is: ");
    print_type_of(&c);
    println!("The value of `z` is: {z}");
    println!("The value of `heart_eyed_cat` is: {heart_eyed_cat}");

    // Compound Types
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of `tup` is: {:?}", tup);
    print!("Type of `tup` is: ");
    print_type_of(&tup);

    // Destructuring a tuple via pattern matching
    let (x, y, z) = tup;

    println!("The value of `x` is: {x}");
    println!("The value of `y` is: {y}");
    println!("The value of `z` is: {z}");
    print!("Type of `y` is: ");
    print_type_of(&y);

    // Destructuring a tuple via numeric accessors
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of `five_hundred` is: {five_hundred}");
    println!("The value of `six_point_four` is: {six_point_four}");
    println!("The value of `one` is: {one}");

    // Arrays
    let a = [1, 2, 3, 4, 5];
    println!("The value of `a` is: {:?}", a);
    print!("Type of `a` is: ");
    print_type_of(&a);

    // Arrays have static length, so can represent things like months
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("The value of `months` is: {:?}", months);
    print!("Type of `months` is: ");
    print_type_of(&months);

    // Initialize array w/same value: 3, length: 5
    let a = [3; 5];
    println!("The value of `a` is: {:?}", a);
    print!("Type of `a` is: ");
    print_type_of(&a);

    // Array indexing
    let first = a[0];
    let second = a[1];
    println!("The value of `a[0]` is: {:?}", first);
    print!("Type of `a[0]` is: ");
    print_type_of(&first);
    println!("The value of `a[1]` is: {:?}", second);
    print!("Type of `a[1]` is: ");
    print_type_of(&second);

    // Array out-of-bounds indexing demo
    // Skip when STDIN is not a tty
    if atty::is(atty::Stream::Stdin) {
        println!("STDIN is a tty... running interactive array index demo");
        // Entering an index > 4 results in panic
        guess_array_demo();
    } else {
        println!("STDIN is NOT a tty... skipping interactive array index demo");
    }

    // Functions
    main2();
}

use std::io;

fn guess_array_demo() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

// Functions
fn main2() {
    another_function(5);
    print_labeled_measurement(5, 'h');
    statements_vs_expressions();
    functions_with_return_values();
    semicolons_are_statements();
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn statements_vs_expressions() {
    let y = 6;
    println!("The value of statement `let y = 6` is: {y}");
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of curly-braced expression `let y = {{let x = 3; x + 1}}` is: {y}");
}

// Expressions do not include ending semicolons.
// If you add a semicolon to the end of an expression,
// you turn it into a statement,
// and it will then not return a value.
fn functions_with_return_values() {
    let retval = five();

    println!("The return value of function `five()` is: {retval}");
}

// function with implicit return value
fn five() -> i32 {
    5
}

fn semicolons_are_statements() {
    let x = plus_one(5);

    println!("The value of `plus_one(5)` is: {x}");
}

fn plus_one(x: i32) -> i32 {
    // Uncomment semicolon & run cargo check to demonstrate compiler error
    x + 1 //;
}
