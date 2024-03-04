# Simple Calculator in Rust

This is a simple command-line calculator written in Rust. It supports basic arithmetic operations: addition, subtraction, multiplication, and division.

## Usage

1. Run the program.
2. When prompted, enter the first number.
3. Next, enter the operation. The operation should be one of the following: "+", "-", "\*", "/".
4. Enter the second number.
5. The program will then calculate the result and display it.

## Code Structure

The program uses an enum `Operation` to represent the arithmetic operations. The `calculate` function takes an `Operation` and calculates the result.

The `main` function handles the user input and output. It first prompts the user to enter the first number, the operation, and the second number. It then constructs an `Operation` based on the user input, calculates the result, and prints it.

## Error Handling

The program will panic if the user enters an operation that is not recognized. It also assumes that the user input is always valid.

## Future Improvements

- Better error handling: Instead of panicking on invalid input, the program could prompt the user to enter the input again.
- Support for more operations: The program could be extended to support more complex operations, such as exponentiation and square roots.
