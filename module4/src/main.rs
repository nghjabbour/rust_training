// Module 4: Collections and Error Handling Deep Dive
// This module explores Rust's collection types and error handling mechanisms in depth:
// - Collection internals and memory layout
// - Performance characteristics and trade-offs
// - UTF-8 handling and string internals
// - Advanced error handling patterns
// - Custom error type design
// Comparisons with other languages are provided to highlight Rust's unique approach.

mod problems;

use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fmt;
use std::mem;

fn main() {
    // Run the practice problems
    problems::run_exercises();

    // ===============================
    // 1. Vector Internals
    // ===============================
    println!("\n1. Vector Internals:");
    println!("------------------");
    // Vec<T> is a triple of:
    // - Pointer to heap-allocated buffer
    // - Length (number of elements)
    // - Capacity (buffer size)

    let mut numbers: Vec<i32> = Vec::with_capacity(10);
    println!("Initial:");
    println!("  Length: {}", numbers.len());
    println!("  Capacity: {}", numbers.capacity());
    println!("  Size in memory: {} bytes", mem::size_of_val(&numbers));

    // Growth pattern demonstration
    println!("\nGrowth pattern:");
    for i in 0..20 {
        numbers.push(i);
        println!(
            "  After push {}: len={}, cap={}",
            i,
            numbers.len(),
            numbers.capacity()
        );
    }

    // Vec vs VecDeque performance comparison
    let mut vec = Vec::new();
    let mut deque = VecDeque::new();

    // Benchmark push_front operations
    let start = std::time::Instant::now();
    for i in 0..10000 {
        vec.insert(0, i); // O(n) operation
    }
    println!("\nVec push_front time: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    for i in 0..10000 {
        deque.push_front(i); // O(1) operation
    }
    println!("VecDeque push_front time: {:?}", start.elapsed());

    // ===============================
    // 2. String Internals
    // ===============================
    println!("\n2. String Internals:");
    println!("-----------------");
    // String is essentially a Vec<u8> that guarantees valid UTF-8

    // UTF-8 encoding demonstration
    let text = "Hello, 世界!";
    println!("Text: {}", text);
    println!("Bytes: {:?}", text.as_bytes());
    println!("Chars: {:?}", text.chars().collect::<Vec<_>>());

    // Memory layout
    let string = String::from(text);
    println!("\nString memory layout:");
    println!("  Pointer: {:p}", string.as_ptr());
    println!("  Length: {} bytes", string.len());
    println!("  Capacity: {} bytes", string.capacity());

    // String vs str comparison
    println!("\nMemory comparison:");
    println!("  String size: {} bytes", mem::size_of::<String>());
    println!("  &str size: {} bytes", mem::size_of::<&str>());

    // UTF-8 validation and handling
    let bytes = vec![0xE4, 0xB8, 0x96]; // UTF-8 encoded '世'
    if let Ok(s) = String::from_utf8(bytes) {
        println!("\nValid UTF-8: {}", s);
    }

    let invalid = vec![0xFF, 0xFF]; // Invalid UTF-8
    match String::from_utf8(invalid) {
        Ok(_) => println!("Valid UTF-8"),
        Err(e) => println!("Invalid UTF-8: {}", e),
    }

    // ===============================
    // 3. HashMap Implementation
    // ===============================
    println!("\n3. HashMap Implementation:");
    println!("------------------------");
    // HashMap uses SipHash-1-3 for DoS resistance
    // Initial capacity is 0, grows on demand

    let mut map = HashMap::with_capacity(10);
    println!("Initial capacity: {}", map.capacity());

    // Demonstrating load factor and resizing
    println!("\nLoad factor behavior:");
    for i in 0..20 {
        map.insert(i, i * i);
        println!(
            "  After insert {}: len={}, cap={}",
            i,
            map.len(),
            map.capacity()
        );
    }

    // Custom key types need Hash + Eq
    #[derive(Hash, Eq, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    let mut point_map = HashMap::new();
    point_map.insert(Point { x: 0, y: 0 }, "origin");
    point_map.insert(Point { x: 1, y: 1 }, "diagonal");

    // Entry API for complex operations
    let point = Point { x: 0, y: 0 };
    let entry = point_map.entry(point);
    entry.and_modify(|v| *v = "ORIGIN").or_insert("new point");

    // ===============================
    // 4. Advanced Error Handling
    // ===============================
    println!("\n4. Advanced Error Handling:");
    println!("------------------------");

    // Custom error type with context
    #[derive(Debug)]
    enum AppError {
        IoError {
            source: std::io::Error,
            path: String,
        },
        ParseError {
            source: ParseError,
            line: usize,
        },
        ValidationError(String),
    }

    impl fmt::Display for AppError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                AppError::IoError { source, path } => {
                    write!(f, "IO error at {}: {}", path, source)
                }
                AppError::ParseError { source, line } => {
                    write!(f, "Parse error at line {}: {}", line, source)
                }
                AppError::ValidationError(msg) => {
                    write!(f, "Validation error: {}", msg)
                }
            }
        }
    }

    impl Error for AppError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                AppError::IoError { source, .. } => Some(source),
                AppError::ParseError { source, .. } => Some(source),
                AppError::ValidationError(_) => None,
            }
        }
    }

    // Error context and chaining
    #[derive(Debug)]
    struct ParseError {
        message: String,
    }

    impl fmt::Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Parse error: {}", self.message)
        }
    }

    impl Error for ParseError {}

    // Function demonstrating error context
    fn process_file(path: &str) -> Result<(), AppError> {
        std::fs::read_to_string(path).map_err(|e| AppError::IoError {
            source: e,
            path: path.to_string(),
        })?;

        Ok(())
    }

    // Error handling patterns
    match process_file("nonexistent.txt") {
        Ok(()) => println!("File processed successfully"),
        Err(e) => {
            println!("Error: {}", e);
            if let Some(source) = e.source() {
                println!("Caused by: {}", source);
            }
        }
    }

    // ===============================
    // 5. Option and Result Patterns
    // ===============================
    println!("\n5. Option and Result Patterns:");
    println!("----------------------------");

    // Combining multiple Options
    let x = Some(2);
    let y = Some(4);
    let sum = x.and_then(|a| y.map(|b| a + b));
    println!("Combined Options: {:?}", sum);

    // Option chaining with early return
    fn process_data(data: Option<i32>) -> Option<i32> {
        // Using ? with Option
        let value = data?;
        let doubled = Some(value * 2)?;
        Some(doubled + 1)
    }

    // Result transformation and mapping
    fn validate_positive(n: i32) -> Result<u32, String> {
        if n < 0 {
            Err("Number must be positive".to_string())
        } else {
            Ok(n as u32)
        }
    }

    let numbers = [-1, 0, 1, 2];
    let results: Vec<_> = numbers.iter().map(|&n| validate_positive(n)).collect();
    println!("Results: {:?}", results);

    // Collecting Results
    let collected: Result<Vec<_>, _> = results.into_iter().collect();
    println!("Collected: {:?}", collected);
}

// Notes on Collections and Error Handling:
//
// 1. Collection Performance
//    - Vec: O(1) push/pop at end, O(n) insert/remove at arbitrary position
//    - VecDeque: O(1) push/pop at both ends
//    - HashMap: O(1) average case operations
//    - String: O(n) concatenation, O(1) push_str to end
//
// 2. Memory Management
//    - Collections own their elements
//    - Capacity management is automatic but can be manually optimized
//    - Memory is contiguous for Vec and String
//    - HashMap uses separate chaining for collisions
//
// 3. Error Handling Philosophy
//    - Explicit over implicit
//    - Recoverable vs unrecoverable errors
//    - Rich error context
//    - Zero-cost abstractions
//
// 4. Comparison with Other Languages
//    - No null pointers (Option<T> instead)
//    - No exceptions (Result<T, E> instead)
//    - UTF-8 only for strings (unlike Java's UTF-16)
//    - HashMap DoS protection built-in

// Try experimenting with these concepts:
// 1. Compare performance of different collections
// 2. Create custom error types with context
// 3. Work with UTF-8 encoded strings
// 4. Use advanced Option/Result combinators
// 5. Implement custom Hash and Eq traits
