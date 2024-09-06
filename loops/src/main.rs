fn main() {
    let mut count = 0;
    'counting_up: loop {
        // 'conting_upはラベル
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break; // 入れ子の内側のループを止める
            }
            if count == 2 {
                break 'counting_up; // 外側のラベルがついたループを止める
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    // 発射！
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    // この書き方だと配列の長さ（index）の指定を間違えた場合にパニックになる
    // またコンパイラが実行時にループの各回ごとに境界値チェックを行うようなコードを追加するため遅くなる
    while index < 5 {
        // 値は{}です
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    // 配列を逆順にみる
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

// loopはラベルをつかって指定停止するか、breakをもちいてスコープ内のloopをめる
// whileは条件を指定して繰り返す
// for...in... は配列をそのまま回せる
