fn main() {
// -------------------------------------------------------------------------
  // 変数と可変性
  let x = 5;
  println!("The value of x is: {}", x);
  // x = 6;
  // println!("The value of x is: {}", x);
  let mut x_mut = 5;
  x_mut = 6;
  println!("{}", x_mut);

  // 定数は大文字のスネークケースで型注釈とconstでの宣言必須
  // 関数の呼び出し結果や実行時に評価される値はセットできない
  const X_NUM: u32 = 5;
  println!("{}", X_NUM);
// -------------------------------------------------------------------------
  // シャドーイング：変数に別の値をいれる。
  // letで変数宣言
  // letで宣言した変数に再代入するときにもletを使う
  let y = 3;
  let y = y + 1;
  {
      let y = y * 2;
      println!("The value of y in the inner scope is: {}", y);
  }
  println!("The value of y is: {}", y);

  // mutとの違いはletを使用すると新しい変数が生成される。よって型の書き換えも可能。
  // mutでは型は可変ではない。letも型を変えるというよりは新しく生成している。
  let name = "nosuke";
  println!("{}", name);
  let name = name.len();
  println!("{}", name);
// -------------------------------------------------------------------------

// let num : i8 = 300; // erro
// let num : i8 = 127 // ok
  // addition
  // 足し算
  let sum = 5 + 10;
  println!("sum {}", sum);

  // subtraction
  // 引き算
  let difference = 95.5 - 4.3;
  println!("difference {}", difference);

  // multiplication
  // 掛け算
  let product = 4 * 30;
  println!("product {}", product);

  // division
  // 割り算
  let quotient = 56.7 / 32.2;
  println!("quotiebt {}", quotient);
  let floored = 2 / 3;
  println!("floored {}", floored);
  

  // remainder
  // 余り
  let remainder = 43 % 5;
  println!("remainder {}", remainder);

// -------------------------------------------------------------------------
  // 複合型（タプル型、配列型）
  // タプル型
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (a, b, c) = tup;
  println!("{}", c);
  let x: (i32, f64, u8) = (500, 6.4, 1);

  let five_hundred = x.0;

  let six_point_four = x.1;

  let one = x.2;
  println!("500:{}, 6.4:{}, 1:{}", five_hundred, six_point_four, one);
  
  // 配列型
  // Rustの配列は固定長
  // 1年の月の名前を扱うプログラムです。そのようなプログラムで、 月を追加したり削除したりすることまずないので、配列を使用
  // 標準ライブラリのベクタ型は柔軟に伸縮できる
  let a = [1, 2, 3, 4, 5];
  let a: [i32; 5] = [1, 2, 3, 4, 5]; // 型注釈の例
  let a = [3; 5]; // こんな感じでも書ける
  // 各要素へのアクセスは以下のようにできる
  let a = [1, 2, 3, 4, 5];
  let first = a[0];
  let second = a[1];
// -------------------------------------------------------------------------
  println!("{}", sum_x_y_1(1, 2));
  print_a('A');
}

fn sum_x_y_1 (x: i8, y: i8)->i8 {
  let result = x + y; // 終端に；は文として処理
  result + 1 // 終端に；ないので式として評価される
}

fn print_a (a: char) {
  println!("{}", a)
}
