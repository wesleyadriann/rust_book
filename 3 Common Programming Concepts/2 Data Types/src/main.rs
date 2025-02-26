fn main() {
    // Data Types
    // let guess = "42".parse().expect("Not a number!");
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("guess: {guess}");

    // Integer Types
    let decimal: u16 = 1_000;
    let hex: u16 = 0xff;
    let octal = 0o77;
    let bin = 0b1111_0000;
    let byte = b'A';

    println!("decimal: {decimal} | hex: {hex} | octal: {octal} | binary: {bin} | byte {byte}");

    // Floating-Point Types
    let float32: f32 = 2.0;
    let float64 = 3.0;

    println!("float32: {float32} | float64: {float64}");

    // Numeric Operations
    let sum = 1 + 2;

    let diff = 5.5 - 4.3;

    let product = 4 * 20;

    let quotient = 5.5 / 2.2;
    let quotient_int = 5 / 2;

    let remainder = 10 % 3;

    println!("sum: {sum} | diff: {diff} | product: {product} | quotient: {quotient} | quotient int: {quotient_int} | remainder: {remainder}");

    // Boolean Type
    let t = true;
    let f: bool = false;

    println!("true: {t} | false {f}");

    // Char type
    let c1 = 'c';
    let c2: char = 'ℤ';
    let c3 = '😻';

    println!("char 1: {c1} | char 2: {c2} | char 3: {c3}");

    // Tuple Type

    let tup1: (i16, f32, u8) = (-150, 7.9, 1);
    
    let t10 = tup1.0;
    let t11 = tup1.1;
    let t12 = tup1.2;

    println!("tup1.0: {t10} | tup1.1: {t11} | tup1.2: {t12}");

    let tup2 = (20, 6.6, 1);
    let (t20, t21, t22) = tup2;

    println!("tup2.0: {t20} | tup2.1: {t21} | tup3.2: {t22}");


    // Array Type

    let _arr1 = [1, 2, 3];

    let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let _arr2: [u16; 5] = [1,2,3,4,5]; 
    let arr3 = [3; 5]; 

    let elem0 = arr3[0];
    let elem1 = arr3[1];

    println!("elem0: {elem0} | elem1: {elem1}");

}
