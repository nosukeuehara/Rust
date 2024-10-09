fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let s = "hello ele";

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // 不変な参照がされている

    s.clear(); // error! （エラー！）

    println!("the first word is: {}", word);
}
