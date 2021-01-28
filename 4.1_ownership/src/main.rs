fn main() {
    /*
    スタック or ヒープ
    スタック
      - last in, first out
      - データは全て既知の固定サイズでないといけない
      - 構造上高速にデータにアクセス可能
    ヒープ
      - コンパイル時にデータのサイズがわからない or サイズが可変の場合に格納される
      - allocating on the heap(十分な領域を確保 -> その場所を使用中にする -> ポインタを返す)
     */

    //実引数と関数のローカル変数がスタックにのる
    //関数の実行が完了するとそれらの値はスタックから取り除かれる

    //文字列リテラルの内容がコンパイル時に確定しているので、バイナリに直接ハードコードされる
    let s = "hello";
    {
        //実行時に必要なメモリをOSに要求
        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{}", s);
    } //スコープ抜ける -> drop関数を内部的に呼び出し対象の変数がヒープに確保しているメモリを返還する

    //ムーブ
    let s1 = String::from("hello");
    let s2 = s1; //s1の参照が無効化される
    //println!("{}, world!", s1);

    //clone
    let s1 = String::from("hello");
    let s2 = s1.clone(); //deep copy(ヒープのデータがコピーされている)
    println!("s1 = {}, s2 = {}", s1, s2);

    //スタックのみのデータのコピー
    let x = 5;
    let y = x; //スタックに保存され、値をコピーするのも高速。変数xは無効化されない
    println!("x = {}, y = {}", x, y);

    //所有権と関数
    let s = String::from("hello");
    take_ownership(s);

    let x = 5;
    make_copy(x);
    println!("not drop x: {}", x);

    //戻り値とスコープ
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    //値を返すことでもs2の所有権は移動
    let s3 = takes_and_gives_back(s2);
    //println!("moved s2 {}", s2);
}

fn take_ownership(str: String) { //str in scope
    println!("{}", str);
} //drop str

fn make_copy(int: i32) { //int in scope
    println!("{}", int);
} //intはdropされない

fn gives_ownership() -> String {
    let str = String::from("hello");
    str
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // a_stringが返され呼び出し元関数にムーブされる
}