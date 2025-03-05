use std::{fs::File, io::ErrorKind};

fn main() {
    let v = vec![1, 2, 3];
    let data = v.get(99);
    match data {
        Some(v) => println!("{v}"),
        None => println!("None"),
    }

    let greeting_file_result = File::open("hello.txt");
    /* 
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(err) => panic!("Problem operation the file: {:?}", err),
    };
    */
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    println!("File created");
                    fc
                },
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_errro => {
                panic!("Problem operationg the file: {:?}", other_errro);
            }
        },
    };

    let greeting_file2 = File::open("hello2.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    //let greeting_file_unwrap = File::open("hello3.txt").unwrap();
    let greeting_file_expect = File::open("hello3.txt").expect("Not found file hello3.txt");
}