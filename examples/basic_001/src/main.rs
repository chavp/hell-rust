fn main() {
    //if_expression();
    //handling_multiple_condition_with_else_if();
    //using_if_in_a_let_statement();
    //repetition_with_loops();
    //returning_values_from_loops();
    //loop_labels_to_disambiguate_between_multiple_loops();
    //conditional_loops_with_while();
    looping_through_a_collection_with_for();
}

// Control Flow
// if Expressions
fn if_expression() {
    println!("# if Expressions");

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

// Handling Multiple Conditions with else if
fn handling_multiple_condition_with_else_if() {
    println!("# Handling Multiple Conditions with else if");
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if  number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// Using if in a let Statement
fn using_if_in_a_let_statement() {
    println!("# Using if in a let Statement");
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

// Repetition with Loops
fn repetition_with_loops() {
    println!("# Repetition with Loops");

    let mut count = 0;
    loop {
        count = count + 1;
        println!("again: {count}");
    }
}

// Returning Values from Loops
fn returning_values_from_loops() {
    println!("# Returning Values from Loops");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

// Loop Labels to Disambiguate Between Multiple Loops
fn loop_labels_to_disambiguate_between_multiple_loops() {
    println!("# Loop Labels to Disambiguate Between Multiple Loops");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("ramaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

// Conditional Loops with while
fn conditional_loops_with_while() {
    println!("# Conditional Loops with while");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

const LIFTOFF: &str = "LIFTOFF!!!";

// Looping Through a Collection with for
fn looping_through_a_collection_with_for() {
    println!("# Looping Through a Collection with for");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    println!("{LIFTOFF}");
    for element in a {
        println!("the value is: {element}");
    }
    println!("{LIFTOFF}");
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("{LIFTOFF}");
}