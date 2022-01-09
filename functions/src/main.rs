fn main() {
    println!("Hello, world!");

    let x = five();

    println!("The value of x is: {}", x);

    // doesn't work
    // let x = (let y = 6);
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let z = plus_one(5);

    println!("The value of z is: {}", z);

    another_function();
    different_function(5);
    print_labeled_measurement(5,'h');
}

fn five() -> i32 {5}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function() {
    println!("Another function.");
}

fn different_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}