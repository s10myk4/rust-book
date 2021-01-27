fn main() {
    //if formula
    let num = 6;

    if num % 4 == 0 {
        println!("number is divisible by 4");
    } else if num % 3 == 0 {
        println!("number is divisible by 3");
    } else if num % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3, or 2");
    }

    let cond = true;
    let num = if cond {
        5
    } else {
        6
    };
    println!("The value of number is: {}", num);

    //loop
    let mut num = 3;
    while num != 0 {
        println!("{}!", num);
        num = num - 1;
    }

    let a = [10, 20, 30, 40, 50];
    //let mut index = 0;
    // コンパイラが実行時にループの各回ごとに境界値チェックを行うようなコードを追加するため遅い
    //while index < 5 {
    //    println!("the value is: {}", a[index]);
    //    index = index + 1;
    //}
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for num in (1..4).rev() {
        println!("{}!", num);
    }
}
