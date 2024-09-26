fn main() {
    let _ = match is_even_caller(1) {
        Ok(r) => {
            println!("{}", r);
        }
        Err(e) => {
            println!("{}", e);
        }
    };
}

#[derive(Debug)]
struct MyError {
    message: String,
}

impl MyError {
    fn new(message: &str) -> Self {
        MyError {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl std::error::Error for MyError {
    fn description(&self) -> &str {
        &self.message
    }
}

fn is_even_caller(n: i32) -> Result<bool, Box<dyn std::error::Error>> {
    let result_2 = is_even(n)?;
    Ok(result_2)
}

fn is_even(n: i32) -> Result<bool, Box<dyn std::error::Error>> {
    if n % 2 == 0 {
        Ok(true)
    } else {
        Err(Box::new(MyError::new("Not even")))
    }
}
