# Propagating Errors Example

Nothing here, Just to show how to propagate errors in Rust.

## How to propagate errors in Rust?

1. First create your custom error as struct and include the `Debug` trait.

```rust
#[derive(Debug)]
struct MyError {
    message: String,
}
```

2. Implement the `std::fmt::Display` trait for your custom error.

```rust
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
```

3. Implement the `std::error::Error` trait for your custom error.

```rust
impl std::error::Error for MyError {
    fn description(&self) -> &str {
        &self.message
    }
}
```

4. Return your custom error from the function where you want to propagate the error.

```rust
fn my_function() -> Result<(), Box<dyn std::error::Error>> {
    Err(MyError {
        message: "This is my custom error".to_string(),
    })
}
```

5. In the caller function, match the error and handle it.

```rust
fn caller_function() -> Result<(), Box<dyn std::error::Error>> {
    let result = my_function()?;
    Ok(result)
}
```
