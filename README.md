# computorv1

`computorv1` is a polynomial equation solver that efficiently handles linear and quadratic equations. It utilizes the Logos Lexer for lexical analysis and Clap for command-line argument parsing, ensuring a seamless and user-friendly experience.

## Installation

To install `computorv1`, follow these steps:

1. Ensure that you have Rust installed on your system. If not, install Rust from [the official website](https://www.rust-lang.org/learn/get-started).
2. Clone the `computorv1` repository:
3. Navigate to the cloned directory:
4. Build the project using Cargo, Rust's package manager and build system:
5. Navigate to the binary:
   ```bash
   git clone https://github.com/0xSpyC/computorv1.git
   cd computorv1
   cargo build --release
   cd target/debug
   ```
## Usage

To solve a polynomial equation using `computorv1`, use the following command syntax:

  ```bash
  ./computorv1 "<polynomial>"
  ```
A polynomial is an equation with the Following Form : $P(x) = a_n x^n + a_{n-1} x^{n-1} + \cdots + a_2 x^2 + a_1 x + a_0$

Replace <polynomial> with the equation you want to solve. For example:

  ```bash
  ./computorv1 "2 * X^2 + 4 * X + 1 = 0"
  ```

This command will output the solution to the specified quadratic equation.


## Testing

`computorv1` comes with a suite of tests to ensure its reliability and accuracy. To run these tests, use the following command:

  ```bash
  cargo test
  cargo test --test <specific_test>
  ```

This will execute all tests defined in the project, verifying the functionality of various components including the polynomial solver, Logos Lexer, polynomial parser.
Specific tests are : polynomial, tokenize, parsing 

For further information or if you encounter any issues submit an issue on the GitHub repository.
