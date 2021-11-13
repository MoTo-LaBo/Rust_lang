pub fn run() {
    // 所有権の move が適用される場合
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);

    // Copy される場合
    let i1 = 1;
    let i2 = i1;
    println!("{} {}", i1, i2);

    // i1, i2 address 確認
    println!("Stack address of i1 is: {:p}", &i1);
    println!("Stack address of i2 is: {:p}", &i2);

    // 文字列 slice : move は起こらず copy されている
    let sl1 = "literal";
    let sl2 = sl1;
    println!("{} {}", sl1, sl2);
    println!("Stack address of sl1 is: {:p}", &sl1);
    println!("Stack address of sl2 is: {:p}", &sl2);

    // deep copy : clone() method
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("Stack address of s3 is: {:p}", &s3);
    println!("Stack address of s4 is: {:p}", &s4);

    // Heap の先頭　address を表示
    println!("Heap memory address of s3 is: {:?}", s3.as_ptr());
    println!("Heap memory address of s4 is: {:?}", s4.as_ptr());

    // move が起こっていないので
    println!("{} {}", s3, s4);

    // take_ownership に渡す
    let s5 = String::from("hello");
    println!("Stack address of s5 is: {:p}", &s5);
    println!("Heap memory address of s5 is: {:?}", s5.as_ptr());
    println!("Len is {}", s5.len());
    println!("Cap is {}", s5.capacity());
    take_ownership(s5);
    // println!("{}", s5); -> take_ownership に所有権が移る為 println! できない

    // take_giveback_ownership : 所有権が2回移動する -> s6 -> s -> s7 へ (drop権限保持)
    let s6 = String::from("hello");
    println!("Stack address of s6 is: {:p}", &s6);
    println!("Heap memory address of hello: {:?}", s6.as_ptr());
    println!("Len is {}", s6.len());
    let s7 = take_giveback_ownership(s6);
    println!("Stack address of s7 is: {:p}", &s7);
    println!("Heap memory address of hello: {:?}", s7.as_ptr());
    println!("Len is {}", s7.len());

    // calculate_length : 参照
    let s8 = String::from("hello");
    let len = calculate_length(&s8);
    println!("The length of '{}' is {}.", s8, len);

    // reference(参照)は data を変更する用途にも使用できる mutable reference
    let mut s9 = String::from("hello");
    // &mut
    change(&mut s9);
    println!("{}", s9);

    // immutable reference は複数作成することができる
    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} {} {}", s10, r1, r2);

    // mutable と immutable は共存できない
    // let mut s10 = String::from("hello");
    // let r1 = &s10;
    // let r2 = &mut s10;
    // println!("{} {}", r1, r2)

    let mut s11 = String::from("hello");
    let r1 = &mut s11;
    println!("{}", r1);
    println!("{}", s11);

    let mut s12 = String::from("hello");
    let r1 = &s12;
    let r2 = &s12;
    println!("{} and {}", r1, r2);

    let r3 = &mut s12;
    *r3 = String::from("hello_updated");
    println!("{}", s12);
}
// take_owership に所有権が移譲したのでスコープを終えたら解放される
fn take_ownership(s: String) {
    println!("Stack address of s is: {:p}", &s);
    println!("Heap memory address of s is: {:?}", s.as_ptr());
    println!("Len is {}", s.len());
    println!("Cap is {}", s.capacity());
    println!("{}", s);
}

// take_giveback_ownership -> return はセミコロンが無いもの
fn take_giveback_ownership(s: String) -> String {
    s
}

// calculate_length : 引数で受け取った length(長さを出したくれる)
// 参照： & を使用
fn calculate_length(s: &String) -> usize {
    s.len()
}

// mutable reference(参照) を受け取って text を追加する method
fn change(s: &mut String) {
    s.push_str("_world");
}
