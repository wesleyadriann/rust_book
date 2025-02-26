fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Thee hours: {THREE_HOURS_IN_SECONDS}");

    // let x = 5; // immutable
    let mut x = 5; // mutable

    println!("The value of x is: {x}");
    x = 6;

    println!("The value of x is: {x}");


    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // let mut spaces = "    ";
    let spaces = "    ";
    let spaces = spaces.len();

    println!("Spaces: {spaces}");
    
}
