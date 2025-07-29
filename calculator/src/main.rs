use std::io::stdin;
use std::num::ParseFloatError;

fn main() {
    let mut input = String::new();

    println!("Please enter the first operand:");
    stdin().read_line(&mut input).unwrap();
    let op1: f64 = match validation_operand(&input) {
        Ok(num) => num,
        Err(msg) => {
            println!("Error: {}", msg);
            return;
        }
    };

    input.clear();

    println!("Please enter the operation:");
    stdin().read_line(&mut input).unwrap();
    let op = match validate_operation(&input) {
        Ok(op) => op,
        Err(_) => return,
    };

    input.clear();

    println!("Please enter the second operand:");
    stdin().read_line(&mut input).unwrap();
    let op2: f64 = match validation_operand(&input) {
        Ok(num) => num,
        Err(msg) => {
            println!("Error: {}", msg);
            return;
        }
    };

    let operation =  create_operation(op1,&op, op2);

    let result = Operation::calculate(operation);
    println!("Result {}", result);
}

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

impl Operation {
    fn calculate(op: Operation) -> f64 {
        match op {
            Operation::Add(a, b) => a + b,
            Operation::Subtract(a, b) => a - b,
            Operation::Multiply(a, b) => a * b,
            Operation::Divide(a, b) => {
                if (b == 0.0) {
                    panic!("Cannot divide by 0");
                }
                a / b
            }
        }
    }
}

fn validation_operand(operand: &str) -> Result<f64, ParseFloatError> {
    operand.trim().parse::<f64>()
}

fn validate_operation(operation: &str) -> Result<String, String> {
    match operation.trim() {
        "+" | "-" | "*" | "/" => Ok(operation.trim().to_string()),
        _ => Err("Invalid operation! Use +, -, *, or /".to_string()),
    }
}

fn create_operation(op1: f64, operator: &str, op2: f64) -> Operation {
    match operator {
        "+" => Operation::Add(op1, op2),
        "-" => Operation::Subtract(op1, op2),
        "*" => Operation::Multiply(op1, op2),
        "/" => Operation::Divide(op1, op2),
        _ => panic!("Unknown operation"),
    }
}
