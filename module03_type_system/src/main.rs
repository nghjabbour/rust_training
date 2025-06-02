// Module 3: Type System Deep Dive
// This module explores Rust's rich type system, including:
// - Memory layout and alignment of custom types
// - Zero-cost abstractions with enums
// - Advanced pattern matching
// - Type safety and null safety
// - Error handling patterns
// Comparisons with other languages are provided to highlight Rust's unique approach.

mod problems;

use std::mem;

fn main() {
    // Run the practice problems
    problems::run_exercises();

    // ===============================
    // 1. Struct Memory Layout
    // ===============================
    println!("\n1. Struct Memory Layout:");
    println!("---------------------");

    // Regular struct with named fields
    #[derive(Debug)]
    struct User {
        username: String,   // Heap-allocated, 24 bytes (ptr, capacity, length)
        email: String,      // Heap-allocated, 24 bytes
        sign_in_count: u64, // 8 bytes
        active: bool,       // 1 byte + padding
    }

    // Memory layout analysis
    println!("User struct size: {} bytes", mem::size_of::<User>());
    println!("User struct alignment: {} bytes", mem::align_of::<User>());

    // Field ordering for optimal memory layout
    #[derive(Debug)]
    struct OptimalUser {
        // Group fields by size for optimal packing
        sign_in_count: u64, // 8 bytes
        username: String,   // 24 bytes
        email: String,      // 24 bytes
        active: bool,       // 1 byte
                            // Total: 57 bytes, but will be padded to maintain alignment
    }

    println!("OptimalUser size: {} bytes", mem::size_of::<OptimalUser>());

    // ===============================
    // 2. Zero-Cost Abstractions
    // ===============================
    println!("\n2. Zero-Cost Abstractions:");
    println!("------------------------");

    // Enum memory layout - only needs space for largest variant plus discriminant
    #[derive(Debug)]
    enum Message {
        Quit,                       // 0 bytes + discriminant
        Move { x: i32, y: i32 },    // 8 bytes + discriminant
        Write(String),              // 24 bytes + discriminant
        ChangeColor(i32, i32, i32), // 12 bytes + discriminant
    }

    println!("Message enum size: {} bytes", mem::size_of::<Message>());
    println!("Message alignment: {} bytes", mem::align_of::<Message>());

    // Null pointer optimization
    // Option<&T> and Option<Box<T>> are the same size as &T and Box<T>
    let reference: Option<&i32> = None;
    println!("Option<&i32> size: {} bytes", mem::size_of_val(&reference));
    println!("&i32 size: {} bytes", mem::size_of::<&i32>());

    // ===============================
    // 3. Advanced Pattern Matching
    // ===============================
    println!("\n3. Advanced Pattern Matching:");
    println!("--------------------------");

    // Complex enum for demonstration
    #[derive(Debug)]
    enum Value {
        Number(f64),
        String(String),
        Array(Vec<Value>),
        Object(std::collections::HashMap<String, Value>),
    }

    // Creating a complex nested structure
    let value = Value::Object({
        let mut map = std::collections::HashMap::new();
        map.insert("name".to_string(), Value::String("Alice".to_string()));
        map.insert("age".to_string(), Value::Number(30.0));
        map.insert(
            "hobbies".to_string(),
            Value::Array(vec![
                Value::String("reading".to_string()),
                Value::String("coding".to_string()),
            ]),
        );
        map
    });

    // Advanced pattern matching with guards and bindings
    fn process_value(val: &Value) {
        match val {
            Value::Number(n) if *n > 0.0 => println!("Positive number: {}", n),
            Value::String(s) if s.len() > 5 => println!("Long string: {}", s),
            Value::Array(arr) if !arr.is_empty() => {
                println!("Non-empty array of length {}", arr.len())
            }
            Value::Object(map) => {
                if let Some(Value::String(name)) = map.get("name") {
                    println!("Found name: {}", name);
                }
            }
            _ => println!("Other value: {:?}", val),
        }
    }

    process_value(&value);

    // ===============================
    // 4. Type Safety and Null Safety
    // ===============================
    println!("\n4. Type Safety and Null Safety:");
    println!("----------------------------");

    // Option<T> vs null in other languages
    fn find_user(id: i32) -> Option<String> {
        if id > 0 {
            Some(format!("User {}", id))
        } else {
            None // Explicit absence of value
        }
    }

    // Safe handling of optional values
    match find_user(1) {
        Some(user) => println!("Found user: {}", user),
        None => println!("User not found"),
    }

    // Combinator pattern with Option
    let user_upper = find_user(1)
        .map(|name| name.to_uppercase())
        .unwrap_or_else(|| "Guest".to_string());
    println!("User (uppercase): {}", user_upper);

    // ===============================
    // 5. Error Handling Patterns
    // ===============================
    println!("\n5. Error Handling Patterns:");
    println!("-----------------------");

    // Custom error type
    #[derive(Debug)]
    enum ServiceError {
        NotFound(String),
        InvalidInput { field: String, message: String },
        DatabaseError(String),
    }

    // Function that returns Result with custom error
    fn process_request(input: &str) -> Result<String, ServiceError> {
        if input.is_empty() {
            return Err(ServiceError::InvalidInput {
                field: "input".to_string(),
                message: "Input cannot be empty".to_string(),
            });
        }

        if input == "error" {
            return Err(ServiceError::DatabaseError("Connection failed".to_string()));
        }

        Ok(format!("Processed: {}", input))
    }

    // Error handling with pattern matching
    match process_request("") {
        Ok(result) => println!("Success: {}", result),
        Err(ServiceError::InvalidInput { field, message }) => {
            println!("Invalid input in {}: {}", field, message)
        }
        Err(ServiceError::DatabaseError(msg)) => println!("Database error: {}", msg),
        Err(ServiceError::NotFound(item)) => println!("Not found: {}", item),
    }

    // Using the ? operator for error propagation
    fn complex_operation(input: &str) -> Result<String, ServiceError> {
        let result = process_request(input)?;
        Ok(format!("Complex operation completed: {}", result))
    }

    if let Err(e) = complex_operation("error") {
        println!("Complex operation failed: {:?}", e);
    }
}

// Notes on Rust's Type System:
//
// 1. Memory Layout
//    - Structs are laid out sequentially in memory
//    - Fields are aligned based on their type requirements
//    - Enums use tagged unions with optimized layout
//    - Zero-cost abstractions through compile-time optimization
//
// 2. Type Safety
//    - No implicit conversions
//    - No null values (Option<T> instead)
//    - Exhaustive pattern matching
//    - Compile-time guarantees
//
// 3. Error Handling
//    - Result<T, E> for recoverable errors
//    - panic! for unrecoverable errors
//    - ? operator for ergonomic error propagation
//    - Custom error types for domain-specific errors
//
// 4. Zero-Cost Abstractions
//    - High-level features compile to efficient machine code
//    - No runtime overhead for unused features
//    - Pattern matching optimized by LLVM
//    - Enum layout optimizations (null pointer optimization)
//
// 5. Comparison with Other Languages
//    - No garbage collector (unlike Java/Go)
//    - No null pointers (unlike C/C++)
//    - No exception handling overhead (unlike C++/Java)
//    - Compile-time guarantees (unlike dynamic languages)

// Try experimenting with these concepts:
// 1. Create structs with different field orderings and compare sizes
// 2. Implement complex enums with various variant types
// 3. Use advanced pattern matching with guards and bindings
// 4. Create custom error types for your domain
// 5. Practice using Option and Result combinators
