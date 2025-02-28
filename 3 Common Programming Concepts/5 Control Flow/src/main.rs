fn main() {

    // if Expressions
    let number = 4;

    if number < 5 {
        println!("The condition is true");
    } else {
        println!("The condition is false");
    }

    let number_three = 3;

    // if number {
    //     println!("The number is 3");
    // }

    if number_three != 0 {
        println!("The number is different of zero")
    }


    // Handling Multiple Conditions with else if

    let number_six = 6;


    if number_six % 4 == 0 {
        println!("The number is divisible by 4");
    } else if number_six % 3 == 0 {
        println!("The number is divisible by 3");
    } else if number_six % 2 == 0 {
        println!("The number is divisible by 2");
    } else {
        println!("The number not is divisible by 4, 3 or 2");
    }


    // Using if in a let Statement

    let condition = true;
    let number_x = if condition { 5 } else { 3 };
    // let number_x = if condition { 5 } else { "six" };

    println!("The number number_x is {number_x}");

    // Repetition with Loops
    // Repeating Code with loop

    // loop {
    //     println!("Loop");
    // }

    // Returning Values from Loops

    {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {result}");
    }

    // Loop Labels to Disambiguate Between Multiple Loops

    {
        let mut counter = 0;

        'counting_up: loop {
            println!("count = {counter}");
            let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }

                if counter == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }

            counter += 1;
        }
        println!("end count = {counter}");
    }

    // Conditional Loops with while

    {
        let mut number = 3;

        while number != 0 {
            println!("{number}!");
            number -= 1;
        }

        println!("loop off");
    }

    // Looping Through a Collection with for

    {
        let a = [10,20,30,40,50];
        let mut index = 0;

        while index < 5 {
            println!("the value is {}", a[index]);
            index += 1;
        }

        for el in a {
            println!("the element is {el}");
        }
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("off");
}
