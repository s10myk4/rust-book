fn main() {
    let s = String::from("hello world");
    let word = first_word(&s[..]);
    println!("first word length is {}", word);

    let str_literal = "string literal";
    let word = first_word(&str_literal[..]);
    let word = first_word(&str_literal); //文字列リテラルはすでにスライスなのでそのまま渡せる
    println!("first word length is {}", word);

    //文字列スライス Stringの一部への参照
    let s = String::from("hello world");
    let hello = &s[..5]; //&s[0..5]と同義
    let world = &s[6..11];

    println!("{} {}", hello, world);

    let len = s.len();
    let slice = &s[3..len];
    let slice1 = &s[3..]; //上と同じ意味
    println!("{} {}", slice, slice1);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        //バイトリテラル表記で空白を検出
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
