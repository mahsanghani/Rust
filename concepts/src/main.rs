use std::io;

fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let mut y = 6;
    println!("The value of y is: {}", y);
    y = 7;
    println!("The value of y is: {}", y);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let z = 5;

    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {}", z);
    }

    println!("The value of z is: {}", z);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);

    // data types

    let guess: u32 = "42".parse().expect("Not a number!");

    let a = 2.0; // f64

    let b: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);

    let a = [3; 5];
    let b = [1, 2, 3, 4, 5];
    let c: [i32; 5] = [1, 2, 3, 4, 5];

    let _first = b[0];
    let _second = b[1];

    let _months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

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

    println!("The value of the element at index {} is: {}", index, element);
}
