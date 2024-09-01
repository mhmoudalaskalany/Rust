mod calculator;
mod say_hello;
fn main() {
    say_hello::say_hello("mahmoud");
    let x = calculator::calculate(10, 5, "+");
    println!("sum operation result is : {}", x);
    let x = calculator::calculate(10, 5, "-");
    println!("Subtraction operation result is : {}", x);
    let x = calculator::calculate(10, 5, "*");
    println!("Multiplication operation result is : {}", x);
    let x = calculator::calculate(10, 5, "/");
    println!("Division operation result is : {}", x);
}
