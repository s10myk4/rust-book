fn main() {
    const MAX_POINTS: u32 = 100_000;

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6 * MAX_POINTS;
    println!("The value of x is: {}", x);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("Space is {}", spaces);
}
