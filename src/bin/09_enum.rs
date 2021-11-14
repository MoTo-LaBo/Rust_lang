// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

// * Use an enum with color names as variants
enum  Color {
    _Red,
    _Yellow,
    Bule
}

// * Use a function to print the color name
// * The function must use the enum as a parameter
fn print_color(my_color: Color) {
    // * Use a match expression to determine which color
    match my_color {
        Color::_Red => println!("Red"),
        Color::_Yellow => println!("Yellow"),
        Color::Bule => println!("Bule"),
    }
}

fn main() {
    print_color(Color::Bule);
}
