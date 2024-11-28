#![allow(unused)]
fn main() {
    #[derive(Debug)] // すぐに州を点検できるように
    enum UsState {
        Alabama,
        Alaska,
        // ... などなど
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    // fn value_in_cents(coin: Coin) -> u32 {
    //     match coin {
    //         Coin::Penny => 1,
    //         Coin::Nickel => 5,
    //         Coin::Dime => 10,
    //         Coin::Quarter => 25,
    //     }
    // }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    let five = Some(5);
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let six = plus_one(five);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
