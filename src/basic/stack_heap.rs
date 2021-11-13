// 列挙型を作成する時は enum を使用していく -> Box型にする事によって compile できるようになる
enum List {
    Node(i32, Box<List>),
    Nil,
}

pub fn run() {
    // let a1: [u8; 9000000] = [1; 9000000];

    // ---------------- Vector型 ---------------- //
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];

    // v1, v2 address 確認
    println!("Stack address of v1 is: {:p}", &v1);
    println!("Stack address of v2 is: {:p}", &v2);

    // v1 の実 data の先頭 pointer(address) の値を確認
    println!("Heap memor address of v1: {:?}", v1.as_ptr());

    // v1 の len, cap 要素数を確認
    println!("Len of v1 is: {}", v1.len());
    println!("Capacity of v1 is: {}", v1.capacity());

    // vector型は要素を動的に追加していく事ができる : 1 の index に 10 を入れ替える事ができる
    // insert (index, 変更後の値)
    v1.insert(1, 10);
    println!("{:?}", v1);

    // remove (index) : 削除したい index
    v1.remove(0);
    println!("{:?}", v1);

    // v1, v3 を結合 : append (v3 の参照を渡す)
    v1.append(&mut v3);
    println!("{:?}", v1);
    println!("{:?}", v3);

    // ---------------- Box pointer ---------------- //
    let t1: (i64, String) = (10, String::from("hello"));
    println!("Stack address of tuple data is: {:p}", &t1);

    // heape の実 data address を表示 : string型は tuple では index 1 なので指定
    println!("Heap address of t1.1: {:p}", t1.1.as_ptr());

    // Len, cap の値も取得
    println!("Len of t1 is: {}", t1.1.len());
    println!("Capacity of v1 is: {}", t1.1.capacity());

    // Box pointer を作成する : String を変更可能にしておく場合は mut を付ける
    let mut b1 = Box::new(t1);
    // * (アスタリスク)を付けると参照はずしを使用して実態を書き換える事ができる
    // tuple に直接 access する事ができる
    (*b1).1 += "world";
    println!("{} {}", b1.0, b1.1);

    // b1 の stack 上の address を確認
    println!("Stack address of Box pointer is: {:p}", &b1);

    // Box pointer の address は heap の先頭の address になっている
    println!("Heap address of tuple data is {:p}", b1);
}
