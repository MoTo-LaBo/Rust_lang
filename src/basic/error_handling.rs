pub fn run() {
    // Option version
    let res1 = divesion_option(5.0, 0.0);
    match res1 {
        Some(x) => println!("Result: {:.3}", x),
        None => println!("Not allowed !!"),
    }

    // Result version
    let res2 = divesion_result(5.0, 1.0);
    match res2 {
        Ok(x) => println!("Result: {:.3}", x),
        Err(e) => println!("{}", e),
    }

    // 例外処理
    let a = [0, 1];
    let res3 = sum(&a);
    match res3 {
        Some(x) => println!("Total is: {}", x),
        None => println!("Out of index!!"),
    }
}
// Option version
fn divesion_option(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

// Result version
fn divesion_result(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("Not allowed !!"))
    } else {
        Ok(x / y)
    }
}

// 例外処理 : 存在しない index に access した場合の回避 -> ?
fn sum(a: &[i32]) -> Option<i32> {
    let a0 = a.get(0)?;
    let a1 = a.get(1)?;
    let a2 = a.get(2)?;
    Some(a0 + a1 + a2)
}
