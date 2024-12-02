use std::io;

#[derive(Debug)]
enum CalcError {
    InvalidInput,
    DivisionByZero,
}

struct Calculator {
    memory: f64,
}

impl Calculator {
    fn new() -> Self {
        Self { memory: 0.0 }
    }

    fn perform_operation(&mut self, input: &str) -> Result<f64, CalcError> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() != 2 {
            return Err(CalcError::InvalidInput);
        }

        let operation = parts[0];
        let value = parts[1].parse::<f64>().map_err(|_| CalcError::InvalidInput)?;

        match operation {
            "+" => {
                self.memory += value;
                Ok(self.memory)
            }
            "-" => {
                self.memory -= value;
                Ok(self.memory)
            }
            "*" => {
                self.memory *= value;
                Ok(self.memory)
            }
            "/" => {
                if value == 0.0 {
                    Err(CalcError::DivisionByZero)
                } else {
                    self.memory /= value;
                    Ok(self.memory)
                }
            }
            _ => Err(CalcError::InvalidInput),
        }
    }
}

fn main() {
    let mut calculator = Calculator::new();
    println!("Enter an operation and a number (e.g., '+ 1', '/ 2') or type 'q' to quit:");

    loop {
        println!("Current memory: {}", calculator.memory);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        if input.trim().eq_ignore_ascii_case("q") {
            break;
        }

        match calculator.perform_operation(&input) {
            Ok(result) => println!("Result: {}", result),
            Err(CalcError::InvalidInput) => println!("Error: Invalid input. Please enter a valid operation and number."),
            Err(CalcError::DivisionByZero) => println!("Error: Division by zero is not allowed."),
        }
    }
}