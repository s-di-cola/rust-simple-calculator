# Rust Calculator Task

A simple command-line calculator built with Rust enums and pattern matching. Performs basic arithmetic operations (+, -, *, /).

## What it does

- Asks user for two numbers and an operation
- Uses enums to represent different operations
- Pattern matching to execute the right calculation
- Prints the result

## How to run

```bash
cargo run
```

## Example usage

```
Enter first number: 10
Enter operation (+, -, *, /): +
Enter second number: 5
Result: 15
```

## Key concepts shown

- **Enums**: Defining different operation types (Add, Subtract, Multiply, Divide)
- **Pattern matching**: Using `match` to handle different enum variants
- **User input**: Reading and parsing input from console
- **Error handling**: Basic input validation

## Operations supported

- `+` Addition
- `-` Subtraction
- `*` Multiplication
- `/` Division

## Files

- `src/main.rs` - Calculator implementation with enums and pattern matching