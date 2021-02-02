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

fn plus_one(x: Option<i32>) -> Option<i32> {
    //パターンの表現力とすべてのパターンを網羅しているかをコンパイラが検出してくれるのがとても強力
    match x {
        None => None, //この単位をアーム
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        //state変数が束縛される 列挙子から値を取り出すことができる
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alabama));

    let six = plus_one(Some(5));
    let none = plus_one(None);
}
