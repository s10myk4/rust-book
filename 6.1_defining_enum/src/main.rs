//列挙型
#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    //enumの列挙子に直接データを格納できる
    V6(Ipv6Addr),
}

#[derive(Debug)]
struct Ipv4Addr {
    first: u8,
    second: u8,
    third: u8,
    forth: u8,
}

#[derive(Debug)]
struct Ipv6Addr {
    value: String,
}

fn route(ip_type: IpAddr) {
    println!("{:?}", ip_type)
}

#[derive(Debug)]
enum Message {
    //関連付いたデータなし
    Quit,
    //中に匿名構造体を含む
    Move { x: i32, y: i32 },
    //単独のStringオブジェクトを含む
    Write(String),
    //3つのi32
    ChangeColor(i32, i32, i32),
}


//enumにもメソッド定義できる
impl Message {
    fn echo(&self) {
        println!("{}", "call echo");
    }
}


fn main() {
    let home = IpAddr::V4(Ipv4Addr { first: 127, second: 0, third: 0, forth: 1 });

    let loopback = IpAddr::V6(Ipv6Addr { value: String::from("::1") });

    route(home);
    route(loopback);

    let m = Message::Write(String::from("hello"));
    m.echo();

    let some_number = Some(5);
    let some_str = Some("string");

    println!("{} {}", some_number.unwrap(), some_str.unwrap());
}
