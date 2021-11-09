pub fn run() {
    let st1 = String::from("x");
    let st2 = String::from("y");
    let res1 = get_longest(&st1, &st2);
    println!("{}", res1);

    // x, y の life time が違う場合の例
    let st3 = String::from("x");
    let res2;
    {
        let st4 = String::from("y");
        res2 = get_longest(&st3, &st4);
        println!("{}", res2);
    }
}

// generics lifetimes annotation -> 'a を追加 -> 返り値の短い方の life time を使用
// st1, st2 reference(参照)を受け取ってどちらが大きいかを返す func
fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 引数に何も取らずに,　reference (参照)だけを返す
// function 内で実態を作成して、その reference(参照)を返す
// fn dummy1<'a>() -> &'a str {
//     let s = String::from("demo");
//     &s
// }

// 上下の記述方法は dangling error になるのでNG

// fn dummy2<'a>() -> &'a i32 {
//     let x = 10;
//     &x
// }

// 下記の記述の仕方は、 reference(参照) を返す訳ではないので OK
fn dummy3() -> String {
    let s = String::from("demo");
    s
}
