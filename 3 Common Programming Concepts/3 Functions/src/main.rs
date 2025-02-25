fn main() {
    println!("Hello, world!");

    another_function(5);

    // Parameters
    print_labeled_value(5, 'm');


    // Statements and Expressions

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // Functions with Return Values

    let five_number = five();

    println!("The value of five is: {five_number}");
    
    let six_number = plus_one(5);
    
    println!("The value of six number is: {six_number}");
}

fn another_function(x: u16) {
    println!("the value of x is: {x}");
}

fn print_labeled_value(value: u16, unit_char: char) {
    println!("The value is: {value}{unit_char}");
}

fn five() -> u32 {
    5
}

fn plus_one(x: u32) -> u32 {
    // x + 1;
    x + 1
}