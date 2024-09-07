mod calculator;
fn main() {
    let sum_result = calculator::calculate(10 , 5 , "+");
    let sub_result = calculator:: calculate(10 , 5 ,"-");
    let multiply_result  = calculator::calculate(5 , 5 , "*");
    let division_result = calculator::calculate(10 ,5 , "/");
    println!("Welcome To Calculator");
    println!("sum result {}" , sum_result);
    println!("sub result {}" , sub_result);
    println!("multiply result {}" , multiply_result);
    println!("division result {}" , division_result);
}
