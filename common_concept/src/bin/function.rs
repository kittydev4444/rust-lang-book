fn main() {
    my_function();
    function_with_parameters(42, "Hello, World!");
    let result = returning_function();
    println!("The result of returning_function is: {}", result);
}

fn my_function() {
    println!("This is my function!");
}

fn function_with_parameters(param1: i32, param2: &str) {
    println!("Parameter 1: {}, Parameter 2: {}", param1, param2);
}

fn returning_function() -> i32 {
    42
}
