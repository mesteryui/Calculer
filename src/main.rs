use std::io;
// It's possible define in the program that we use std::io to avoid the need of reference the standard library in the program

fn obtain_number(prompt: &str, custom_err: &str) -> i32 {
    let mut inpt = String::new();
    println!("{}",prompt);
    io::stdin().read_line(&mut inpt).expect(custom_err);
    return inpt.trim().parse::<i32>().unwrap_or(0); // I did this to simplify the normal thing is to use Result and things like that but I want something simple and the 0 doesn't change anything in an operation
}
fn operate(operation: &str, num1: i32, num2:i32) -> i32 {
    match operation {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        "mod" => num1 % num2,
        _ => {
            println!("A correct operation wasn't defined");
            0
        }
    }
}
fn select_operator() -> String {
    println!("Select an operator: +, -, *, /, mod");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).unwrap();
    return operator.trim().to_string();
}

fn main() {
    let num1 = obtain_number("Write the first number: ", "It wasn't possible to read first number");
    let num2 = obtain_number("Write the second number: ", "It wasn't possible to read second number");
    let operator = select_operator();
    println!("The result is {}", operate(operator.as_str(),num1,num2))
}




