fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    //println!マクロを使用してインスタンスを出力
    println!("rectl is {:?}", rect1);
    println!("rectl is {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

//derive注釈でDebugトレイトを利用
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
