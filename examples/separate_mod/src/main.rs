pub mod front_of_house;

use std::fmt::format;

use crate::front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();

    let v1 = vec![1, 2, 3];

    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    println!("v1[0] = {}", &v1[0]);

    let out_index = v1.get(100);
    match out_index {
        Some(i) => println!("out_index[100] = {}", i),
        None => println!("out_index[100] = None"),
    }

    println!("sum(v2) = {}", sum_vec(&v2));

    v2.push(9);
    println!("sum(v2) = {}", sum_vec(&v2));

    let first = &v2[0]; // &v2[0]
    //v2.push(11);
    println!("first = {}", first);
    //v2.push(11);

    let mut v3 = vec![1,2,3,4];
    for i in &mut v3 {
        *i += 10;
    }
    println!("v3 = [{}, {}, {}, {}]", v3[0], v3[1], v3[2], v3[3]);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    for e in row {
        match e {
            SpreadsheetCell::Float(v) => println!("Float = {}", v),
            SpreadsheetCell::Int(v) => println!("Int = {}", v),
            SpreadsheetCell::Text(v) => println!("Text = {}", v),
        }
    }

    let mut s = "hello".to_string();
    //let s = String::from("hello world");
    s.push_str(" world");
    println!("s = {s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{s3}");
    println!("{s2}");

    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");
    let s = format!("{t1}-{t2}-{t3}");
    println!("{s}");

    let hello = "hello";
    let s = &hello[0..2];
    println!("s = {s}");
}

fn sum_vec(data: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for element in data {
        sum += element;
    }
    sum
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}