#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    //..etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let some_u8_value = Some(3);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    };
    //1つのパターンだけを処理したい場合に簡潔に書ける
    if let Some(3) = some_u8_value { println!("three"); }

    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
