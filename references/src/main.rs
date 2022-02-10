fn main() {
    // let s = no_dangle();
    let mut s = "hello";
    let s1 = String::from("hello");

    {
        let r1 = &mut s;
    }

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // change(&mut s);

    // let reference_to_nothing = dangle();
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}