fn main() {
    //浮動小数点
    //基準型はf64
    let x = 2.0;
    let y: f32 = 3.0;

    //数値演算
    let sum = 5 + 10;
    let diff = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    //論理値型
    let t = true;
    let f: bool = false;

    //文字列型
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    //複合型
    //tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    let siz_point_four = tup.1;

    //配列型(固定長)
    let a = [1, 2, 3, 4];
    let first = a[0];
    let second = a[1];
}
