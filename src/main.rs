use std::io::{stdin};

fn main() {
    loop {
        let mut first_number = String::new();
        println!("Please enter first number");
        stdin().read_line(&mut first_number)
           .expect("Failed to read line");
        let first_number: f64 = match first_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
                println!("Please enter only numbers");
                continue
            }
        };
        let mut second_number = String::new();
        println!("Please enter second number");
        stdin().read_line(&mut second_number)
           .expect("Failed to read line");
        let second_number: f64 = match second_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
                println!("Please enter only numbers");
                continue
            }
        };

        let mut operator = String::new();
        println!("Please enter operator name");
        stdin().read_line(&mut operator)
           .expect("Failed to read line");

        let result = match operator.trim() {
            "add" => calculate(Operations::Add(first_number, second_number)),
            "substract" => calculate(Operations::Substract(first_number, second_number)),
            "multiply" => calculate(Operations::Multiply(first_number, second_number)),
            "divide" => calculate(Operations::Divide(first_number, second_number)),
            _ =>{
                println!("Please enter valid operation name (add, substract, mutiply or divide)");
                continue;
            }
        };
        println!("Result is {result}");
    }
}

enum Operations{
    Add(f64,f64),
    Substract(f64,f64),
    Multiply(f64,f64),
    Divide(f64,f64)
}

fn calculate(x:Operations) ->f64{
    let result = match x  {
        Operations::Add(x,y ) => x+y,
        Operations::Substract(x,y ) => x-y,
        Operations::Multiply(x,y ) => x*y,
        Operations::Divide(x,y ) => x/y
    };
    result
}
