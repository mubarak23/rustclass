// enums give you a way of saying a value is one of a possible set of values.

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum UsState {
    Alaska,
    Alabama
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter (UsState),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write (String),
    ChangeColor (i32, i32, i32)
}

// we can implement method on ENUM 
impl Message {
    fn call (&self) {
       //  println!("{}", self);
    }
}

// find value in coin
fn value_in_cents (coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny !!!");
            1
        }
        Coin::Nickel => 10,
        Coin::Dime => 15,
        Coin::Quarter(state) => {
           //  println!("State Quarter from {}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
  //   let loopback = IpAddr::V6(":1").to_string();

    let m = Message::Write(String::from("Demo Day"));
    m.call();
    println!("Hello, world!");
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    // println!("PLUS ONE : {}", six);

    // println!("Plus one : {}", five);

    // Concise Control with if let
    let config_max = Some(3u8);
    match config_max {
        Some (max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // concise with if let
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }


}
