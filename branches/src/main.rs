// 制御フロー
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true"); // 条件は真でした
    } else {
        println!("condition was false"); // 条件は偽でした
    }

    if number != 0 {
        println!("number was something other than zero"); // 数値は0以外の何かです
    }
    // rustの条件式は必ず論理値の必要がある。
    // jsみたいに論理値以外が勝手に論理値に変換されることはない
    // else if のように複数条件がある場合、初めに真になった結果のみ返す。jsと同じ

    let condition = true;
    let number = if condition { 5 } else { 6 };

    // numberの値は、{}です
    println!("The value of number is: {}", number);
}
