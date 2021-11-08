pub mod sub_a;
pub mod sub_b;

// global な変数 : const (変数名は全て大文字) ※ let は関数外では定義する事ができない
const MAX_POINTS: u32 = 100_000;

// スコープ外
// const i2: i64 = 1;
// const i3: i64 = 2;

pub fn run() {
    println!("Here is vars module!!");
    // sub_a::func_a();
    // sub_b::func_b();

    // mutable(可変可能) immutable(可変不可)
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 型の推論 : 定義したが使用していない変数に対しては(アンダーバー)warning が消える
    let _i1 = 3;
    let _f1 = 0.1;

    // system bit の表示
    println!("{}", usize::BITS);

    // const で定義した変数の address の &(番地取得) :p (pointの形で出力)
    println!("Memory address of const is: {:p}", &MAX_POINTS);

    // stack 領域に入る data (スコープ内)
    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("Stack address of i2 is: {:p}", &i2);
    println!("Stack address of i3 is: {:p}", &i3);

    // shadowing : stack 上に常に y の新しい領域が格納されている
    let y = 5;
    println!("Stack address of y is: {:p}", &y);
    let y = y + 1;
    println!("Stack address of y is: {:p}", &y);
    let y = y * 2;
    println!("Stack address of y is: {:p}", &y);
    println!("The value of y is: {}", y);

    // ここでまたスコープを作成して, y = 0 を入れる (スコープ内では y は 0)
    {
        let y = 0;
        println!("Stack address of y is: {:p}", &y);
        println!("The value of y is: {}", y);
    }
    // 上記のスコープを抜けてしまえば y の値は 12 に戻る
    println!("The value of y is: {}", y);

    // tuple 型 : () で複数の要素を並べる事ができる。違う data型を扱う事ができる
    let t1 = (500, 6.4, "dummy");
    // 分割して値を取り出すやり方
    let (_x, _y, _z) = t1;
    // tuple の index を使用して値を取得するやり方
    println!("The value of t1 is: {} {} {}", t1.0, t1.1, t1.2);

    // tuple の入れ子型 : pointer の形で分割して取得できる (ref mut をつける)
    let mut t2 = ((0, 1), (2, 3));
    // 2つ目の tuple を受け取らない場合は _(アンダーバー)で省略できる
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    // 参照はずし * (アスタリスク)を付ける事により pointer へアクセスできる : tuple (0, 1) を書き換える
    *x1_ptr = 5;
    *y1_ptr = -5;
    // tuple, 構造体の様な特殊な型の時は { :? } を指定する事によって出力できる
    println!("{:?}", t2);

    // array (配列) : size や要素数を変更する事はできない
    // compile 時には決定している必要がある : stack につまれていく値になる
    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);
}
