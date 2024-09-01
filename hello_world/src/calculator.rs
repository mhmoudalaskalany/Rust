pub fn calculate(x: i32, y: i32, operation: &str) -> i32 {
    match operation {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        _ => panic!("Invalid Operation {}", operation),
    }
}
