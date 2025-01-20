fn main() {
    /*
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    */
    //let home = IpAddr::V4(String::from("127.0.0.1"));
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(5);

    match some_number {
        Some(i) => { println!("some_number = {}", i); },
        None => { println!("None"); },
    };

    //println!("some_number = {:?}", some_number);
}

enum IpAddrKind {
    V4, 
    V6,
}

/*
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
*/
/*
enum IpAddr {
    V4(String),
    V6(String),
}
*/
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
/*
struct Ipv4Addr { }
struct Ipv6Addr { }
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
*/

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}