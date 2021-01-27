fn main() {

    let x = plus_one(5);
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
   x + 1
}
