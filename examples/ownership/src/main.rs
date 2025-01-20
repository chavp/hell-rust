fn main() {
    //string_clone();
    //owneship();
    //return_values_and_scope();
    //return_multi_value();
    //do_change();
    //do_dangling_reference();
    //do_first_word();
    do_slice();
}

fn string_clone() {
    let s1 = String::from("Hello, world!");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let mut x = 5;
    let y = x;
    x = 10;

    println!("x = {}, y = {}", x, y);
}

fn owneship() {
    let s = String::from("hello");
    let s1 = s.clone();
    takes_ownership(s);
    println!("s1 = {}", s1);

    let x = 10;
    makes_copy(x);
    println!("x = {}", x);
}

fn takes_ownership(some_string: String) {
    println!("some_string = {}", some_string);
}

fn makes_copy(some_i32: i32) {
    println!("some_i32 = {}", some_i32);
}

fn return_values_and_scope() {
    let s1 = gives_ownership();
    println!("s1 = {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s3 = {}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn return_multi_value() {
    let s1 = String::from("hello");
    //let (s2, len) = calculate_len(s1);
    //println!("The length of '{}' is {}.", s2, len);

    let len = calculate_len1(&s1);
    println!("s1 = {}, len = {}", s1, len);
}

fn calculate_len(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_len1(s: &String) -> usize {
    s.len()
} 

fn do_change() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("s = {}", s);

    let r1 = &mut s;
    //let r2 =  s.clone();
    println!("r1 = {}", r1);
}

fn change(some_str: &mut String) {
    some_str.push_str(", world");
}

fn do_dangling_reference() {
    let reference_to_noting = no_dangle();
    println!("reference_to_noting = {}", reference_to_noting);
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
/* 
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
*/

fn do_first_word() {
    let mut s = String::from("hello, world");
    let s1 = first_word(&s);
    println!("s1 = {}", s1);
    s.clear();
    println!("s = {}", s);

    let s2 = String::from("hello world");
    let hello = &s2[0..5];
    let world = &s2[6..11];
    println!("hello = {}, world = {}", hello, world);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return  i;
        }
    }

    s.len()
}

fn do_slice() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}