// struct (構造体)
// generics にする事によって, 整数でも浮動小数でも対応できる
struct Point<T> {
    x: T,
    y: T,
}
// generics に T, U を宣言する
struct PointAnother<T, U> {
    x: T,
    y: U,
}
// impl に struct に追加したい method を記述していく
impl<T, U> PointAnother<T, U> {
    fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
        PointAnother {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    // list の中から一番大き値を取得 : for文を使用
    let number_list = vec![34, 50, 25, 100, 65];
    // let mut largest = number_list[0];
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("The largest is {}", largest);
    // println!("{}", largest_i32(number_list));

    // charctor型の最大値を求める : '' (シングルコーテーション) = char型になる (4bytes)
    let char_list = vec!['a', 'b', 'c', 'd'];
    println!("{}", largest(char_list));
    println!("{}", largest(number_list));

    // struct -> 実態を作成
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.0, y: 2.0 };
    let p3 = PointAnother { x: 1.0, y: 2 };
    let p4 = PointAnother { x: "Rust", y: "a" };

    // mixup method を実行する
    let p5 = p3.mixup(p4);
    println!("{} {}", p5.x, p5.y);
}

// 上記の function を切り分けて記述 : 整数型用
fn largest_i32(list: Vec<i32>) -> i32 {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// char型の最大値を求める function : 整数型のfunc を generics を使用
// generics により整数型でもchar型でも対応できる function を作成できる
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
