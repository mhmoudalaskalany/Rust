mod say_hello;
mod calculator;
fn main() {
    say_hello::say_hello("mahmoud");
    let x = calculator::calculate(5 ,10);
    println!("operation result is : {}" , x);
}
