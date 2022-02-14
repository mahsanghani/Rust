use std::option::Option::{Some, None};

fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    struct IpAddr0 {
        kind: IpAddrKind,
        address: String,
    }

    let home0 = IpAddr0 {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback0 = IpAddr0 {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    enum IpAddr1 {
        V4(String),
        V6(String),
    }

    let home1 = IpAddr1::V4(String::from("127.0.0.1"));

    let loopback1 = IpAddr1::V6(String::from("::1"));

    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home2 = IpAddr2::V4(127, 0, 0, 1);

    let loopback2 = IpAddr2::V6(String::from("::1"));

    struct Ipv4Addr {
        // --snip--
    }

    struct Ipv6Addr {
        // --snip--
    }

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    enum Option<T> {
        None,
        Some(T),
    }

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: std::option::Option<i32> = None;
    let x: i8 = 5;
    let y: std::option::Option<i8> = Some(5);
    // let sum = x + y;

    // println!("{}", four);
    // println!("{}", six);
    // println!("{}", home0);
    // println!("{}", home1);
    // println!("{}", home2);
    // println!("{}", loopback0);
    // println!("{}", loopback1);
    // println!("{}", loopback2);
    // println!("{}", m);
    // println!("{}", some_number);
    // println!("{}", some_string);
    // println!("{}", absent_number);
    // println!("{}", x);
    // println!("{}", y);
    // println!("{}", sum);

    fn route(ip_kind: IpAddrKind) {}
}