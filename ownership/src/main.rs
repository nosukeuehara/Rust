fn main() {
    let s1 = String::from("hello");

    let len2 = calculate_length_2(&s1);
    let len = calculate_length(s1);

    //'{}'の長さは、{}です
    // println!("The length of '{}' is {}.", s1, len); // 上で定義した変数s1はcalculate_length()の引数になったタイミングでdropされているのでエラーになる。
    println!("len: {}, len22: {}", len, len2)
}

fn calculate_length(s: String) -> usize {
    let length = s.len(); // len()メソッドは、Stringの長さを返します

    length
}

// 参照渡し
fn calculate_length_2(s: &String) -> usize {
    let length = s.len(); // len()メソッドは、Stringの長さを返します

    length
}
