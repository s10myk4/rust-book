use std::borrow::Borrow;

fn main() {
    //構造体のインスタンスを生成
    let mut user = build_user(
        String::from("someone@example.com"),
        String::from("user1"),
    );

    user.email = String::from("another@example.com");
    println!("{}", user.email);

    let user2 = User {
        email: String::from("someone2@example.com"),
        username: String::from("user2"),
        ..user //構造体更新記法
    };

    struct Color(i32, i32, i32); //タプル構造体 フィールドに名前がなく型だけ存在する
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    print!("{}", black.0);

    struct UnitStruct(); //ユニット様構造体
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email, //フィールド初期化省略記法
        username,
        active: true,
        sign_in_count: 1,
    }
}