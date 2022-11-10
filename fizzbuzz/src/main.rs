use std::io;

fn main() {
    println!("Enter a value for FizzBuzz to run to:");
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read input");

    let trimmed = value.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => {
            // Run FizzBuzz in main
            let mut fizz_buzz_output: Vec<String> = Vec::new();
            let mut x = 1;

            while x <= i {
                fizz_buzz_output.push(fizz_buzz(x));
                x += 1;
            }
            println!("Output: {:?}", fizz_buzz_output);
        },
        Err(..) => println!("The value you entered was not valid: {}", trimmed)
    }
    
}

fn fizz_buzz(number: i32) -> String {
    if (number % 3 == 0) && (number % 5 == 0) {
        return "FizzBuzz".to_string();
    }
    if number % 3 == 0 {
        return "Fizz".to_string();
    }
    if number % 5 == 0 {
        return "Buzz".to_string();
    }
    return number.to_string();
}