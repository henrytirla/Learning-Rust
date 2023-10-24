fn main() {
    //Variable Type Sign and Unsigned
    //Signed i8, i16,i32,i64,i128, isize
    //Unsigned
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");


    // addition
    let sum = 5 + 10;
    println!("Sum {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("Difference  {difference}");


    // multiplication
    let product = 4 * 30;
    println!("Product {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("Quotient {quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("Truncated {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("Remainder {remainder}");

    let t = true;
    println!("T-Boolean {t}");

    let f: bool = false;
    println!("F-Boolean {f}");

}