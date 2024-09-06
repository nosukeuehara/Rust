fn main() {
    // このコードは動く
    let mut s = String::from("Hello");
    s.push_str(", nosuke");
    println!("{}", s);

    // String型は可変だが、文字列リテラルの場合は長さを変えることはできない
}
