fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("{}", word);
    let word = first_word(&my_string[..]);
    println!("{}", word);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);
    println!("{}", word);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    println!("{}", word);
    let word = first_word(&my_string_literal[..]);
    println!("{}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("{}", word);

    let s = String::from("hello");

    let slice = &s[0..2];
    println!("{}", slice);
    let slice = &s[..2];
    println!("{}", slice);

    let s = String::from("hello");

    let len = s.len();
    println!("{}", len);

    let slice = &s[3..len];
    println!("{}", slice);
    let slice = &s[3..];
    println!("{}", slice);
    let s = String::from("hello");

    let len = s.len();
    println!("{}", len);
    let slice = &s[0..len];
    println!("{}", slice);
    let slice = &s[..];
    println!("{}", slice);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
    println!("Success!");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}