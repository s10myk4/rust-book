fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of {} is {}.", s1, len);

    let mut s = String::from("hello");
    /*
    可変な参照は1つしか持てない
    成約があることで、コンパイル時にデータ競合を防ぐことができる
    - 2つ以上のポインタが同じデータに同時にアクセスする。
    - 少なくとも一つのポインタがデータに書き込みを行っている。
    - データへのアクセスを同期する機構が使用されていない。
     */
    let r1 = &mut s;
    let r2 = &mut s;
    //println!("{} {}", r1, r2);
}

//&記号は参照、引数に参照を取ることを借用という
fn calculate_length(s: &String) -> usize {
    s.len()
} //スコープを抜けてもsは所有権を持っているわけではないのでdropされない

fn change(str: &String) {
    //借用している値も不変なので変更できない
    //str.push_str(", world!");
}

//fn dangle() -> &String {
//    let s = String::from("hello");
//    &s //sへの参照を返す
//} //sはスコープを抜けてdropされるため、ダングリングポインタが発生