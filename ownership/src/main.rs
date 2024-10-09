fn main() {
    let mut s = String::from("Hello");
    println!("{}", s);
    let s2 = calculate_length_2(&mut s);
    println!("変更：{}, 変更前：{}", s2, s)
}

// 参照渡し
fn calculate_length_2(s: &mut String) -> String {
    s.push_str(", world");
    s.to_string()
}
