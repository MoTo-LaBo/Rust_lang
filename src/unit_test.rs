struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn compare_area(&self, other: &Rectangle) -> bool {
        self.width * self.height > other.width * other.height
    }
}

fn double_value(a: i32) -> i32 {
    a * 2
}
fn greeting(name: &str) -> String {
    format!("Hello {} san", name)
}

// =========================== test code ===========================
// cargo test が実行された時だけ compile される
// 通常の build 時には compile されないようになっている
#[cfg(test)]
// mod を付ける事によって file のなかで sub module を作成できる
// mode tests は, unit_test.rs　の一つ下の階層になる -> 取り込む必要がある
// use で値の取り込み -> super で親の内容に access -> ＊（アスタリスク）全ての内容
mod tests {
    use super::*;
    #[test]
    fn test_a_is_larger() {
        let a = Rectangle {
            width: 5,
            height: 5,
        };
        let b = Rectangle {
            width: 3,
            height: 3,
        };
        assert!(a.compare_area(&b));
    }
    #[test]
    fn test_a_is_small() {
        let a = Rectangle {
            width: 3,
            height: 3,
        };
        let b = Rectangle {
            width: 5,
            height: 5,
        };
        assert!(!(a.compare_area(&b)));
    }
    #[test]
    fn test_double() {
        assert_eq!(6, double_value(3));
    }
    #[test]
    fn test_contains_name() {
        let res = greeting("rust");
        assert!(res.contains("rust"));
    }
}
