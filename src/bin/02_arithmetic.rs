// Topic: Basic arithmetic (基本的な算術)
//
// Program requirements:
// * Displays the result of the sum of two numbers
//

// Notes:
// * Use a function to add two numbers togethe
fn sub(a: i32, b: i32) -> i32 {
    a + b
}

// * Use a function to display the result
fn display_result(result: i32) {
// * Use the "{:?}" token in the println macro to display the result    println!("{:?}", result);
    println!("{:?}", result);
}

// * Use a function to add two numbers together


fn main() {
    let sum = 2 + 2;
    println!("{:?}", sum);

    let value = 10 - 5;
    println!("{:?}", value);

    let divesion = 10 / 2;
    println!("{:?}", divesion);

    let mult = 5 * 5;
    println!("{:?}", mult);

    let result = sub(8, 5);
    display_result(result);

    let rem = 6 % 3;
    println!("{:?}", rem);

    let rem2 = 6 % 4;
    println!("{:?}", rem2);

    let result = sub(2, 2);
    display_result(result)

}
