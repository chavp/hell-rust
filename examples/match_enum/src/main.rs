fn main() {
    let p = Coin::Penny;
    println!("1 Penny = {} cents", value_in_cents(&p));

    let q = Coin::Quarter(UsSate::Alabama);
    println!("1 Quarter = {} cents", value_in_cents(&q));

    let x = Some(5);
    let x_plus = plus_one(x);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        //other => move_player(other),
        _ => reroll(),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    match config_max {
        Some(i) => {
            println!("The i is configured to be {}", i);
        },
        _ => (),
    }
}

#[derive(Debug)]
enum UsSate {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsSate),
}

// arm = pattern & some code
fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn add_fancy_hat(){}
fn remove_fancy_hat(){}
//fn move_player(num_spaces: u8){}
fn reroll(){}