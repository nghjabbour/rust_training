// Module 1: Rust Fundamentals
// This module covers fundamental concepts in Rust, explaining not just how to use features,
// but why they exist and how they work under the hood.

mod problems;

fn main() {
    // First, run the practice problems
    problems::run_exercises();

    // ===============================
    // 0. Rust Toolchain and Ecosystem
    // ===============================
    println!("\n0. Rust Toolchain and Ecosystem:");
    println!("-----------------------------");
    // Rust uses a sophisticated toolchain management system:
    // - rustup: manages Rust installations and components
    // - cargo: Rust's package manager and build tool
    // - rustc: the Rust compiler
    //
    // Key cargo commands:
    // cargo new: Creates a new project
    // cargo build: Compiles the project
    // cargo run: Compiles and runs the project
    // cargo test: Runs tests
    // cargo doc: Generates documentation
    // cargo check: Performs compilation checks without producing an executable
    //
    // Rust editions (2015, 2018, 2021) allow language evolution while maintaining backwards compatibility

    // ===============================
    // 1. Variables and Memory Model
    // ===============================
    println!("\n1. Variables and Memory Model:");
    println!("---------------------------");

    // In Rust, every value has an "owner" - this is fundamental to memory safety
    // Variables are immutable by default - this is not just a safety feature,
    // it enables compiler optimizations and makes concurrent code easier to reason about
    let x = 5;
    println!("Immutable x: {}", x);
    // x = 6; // Error: cannot assign twice to immutable variable

    // When we need mutability, we explicitly opt-in
    // This makes code intentions clear and helps prevent bugs
    let mut y = 10;
    println!("Original y: {}", y);
    y = 15;
    println!("Modified y: {}", y);

    // Variable Shadowing
    // Unlike other languages where shadowing might be considered bad practice,
    // Rust embraces it as a way to transform values while reusing names
    let value = "   hello   "; // value is a &str
    let value = value.trim(); // value is still a &str, but trimmed
    let value = value.len(); // value is now a usize
    println!("Transformed value: {}", value);

    // This is different from mut because:
    // 1. Each shadowing creates a new variable (new memory location)
    // 2. We can change types
    // 3. The original variable is completely shadowed (can't access it anymore)

    // ===============================
    // 2. Types and Memory Layout
    // ===============================
    println!("\n2. Types and Memory Layout:");
    println!("--------------------------");

    // Integer Types
    // Rust provides fine-grained control over integer representation
    let a: i32 = 42; // 32 bits, signed (-2^31 to 2^31-1)
    let b: u32 = 42; // 32 bits, unsigned (0 to 2^32-1)
    let c: i8 = 42; // 8 bits, signed (-128 to 127)

    // Integer Overflow Behavior
    // In debug builds: panic
    // In release builds: wrap around
    let mut small: u8 = 255;
    // small += 1;  // Would panic in debug, wrap to 0 in release

    // Memory layout of different types
    use std::mem;
    println!("Size of i32: {} bytes", mem::size_of::<i32>());
    println!("Size of char: {} bytes", mem::size_of::<char>());
    println!("Size of bool: {} bytes", mem::size_of::<bool>());

    // Floating-point Types
    // Rust uses IEEE 754 standard
    let float32: f32 = 3.14159; // 32-bit float
    let float64: f64 = 3.14159; // 64-bit float (default)
    println!("float32 precision: {}", float32);
    println!("float64 precision: {}", float64);

    // Compound Types
    // Tuples: Fixed-length collection of values of different types
    let tuple: (i32, f64, char) = (42, 3.14, 'a');
    println!("Tuple values: ({}, {}, {})", tuple.0, tuple.1, tuple.2);

    // Arrays: Fixed-length collection of same type, stored on stack
    let array: [i32; 4] = [1, 2, 3, 4]; // Type annotation shows [type; size]
    println!("Array length: {}", array.len());
    println!("Array memory size: {} bytes", mem::size_of_val(&array));

    // ===============================
    // 3. Control Flow and Expressions
    // ===============================
    println!("\n3. Control Flow and Expressions:");
    println!("------------------------------");

    // Everything in Rust is an expression (has a value), except:
    // 1. Variable declarations
    // 2. Item declarations (functions, traits, etc.)

    // If expressions (not statements!)
    let number = 7;
    let message = if number % 2 == 0 {
        "even" // Note: no semicolon (this is the expression value)
    } else {
        "odd" // Both branches must return same type
    };
    println!("Number is {}", message);

    // Loop expressions can have values
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // Return value from loop
        }
    };
    println!("Loop result: {}", result);

    // For loops use iterators under the hood
    for i in 0..5 {
        // Range is an iterator
        print!("{} ", i);
    }
    println!();

    // ===============================
    // 4. Functions and Control Flow
    // ===============================
    println!("\n4. Functions and Control Flow:");
    println!("----------------------------");

    // Functions in Rust are zero-cost abstractions
    // The compiler is aggressive about inlining and optimization
    demonstrate_functions();

    // Early returns and the ? operator
    let result = early_return_example(5);
    println!("Early return result: {:?}", result);

    // Expression-based nature of Rust
    let computed = {
        let a = 5;
        let b = 10;
        a + b // Last expression becomes block value
    };
    println!("Computed value: {}", computed);
}

// Function demonstrating various Rust features
fn demonstrate_functions() {
    // Function parameters are patterns
    fn add((x, y): (i32, i32)) -> i32 {
        x + y // Expression return (no semicolon)
    }

    // Diverging functions (never return)
    fn forever() -> ! {
        loop {
            // Never returns
        }
    }

    // Function pointers are zero-sized
    let fn_ptr: fn(i32) -> i32 = square;
    println!(
        "Size of function pointer: {} bytes",
        std::mem::size_of_val(&fn_ptr)
    );
}

// Function demonstrating early returns
fn early_return_example(x: i32) -> Result<i32, String> {
    if x < 0 {
        return Err("Number must be positive".to_string());
    }
    Ok(x * 2)
}

// Function with explicit return type
fn square(x: i32) -> i32 {
    // Expression-based return
    x * x
}

// Function returning multiple values via tuple
fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

// Notes on Rust's Design Philosophy:
//
// 1. Zero-Cost Abstractions
//    - High-level features compile to efficient low-level code
//    - No runtime overhead for unused features
//    - No garbage collector needed
//
// 2. Memory Safety
//    - Ownership system prevents memory bugs at compile time
//    - No null pointers
//    - No data races
//
// 3. Practical Design
//    - Features must have clear use cases
//    - Focus on solving real-world problems
//    - Strong support for systems programming
//
// 4. Concurrency
//    - "Fearless concurrency" through type system
//    - Thread safety guaranteed at compile time
//    - Efficient async/await support
//
// 5. Tooling
//    - Built-in dependency management
//    - Integrated testing framework
//    - Documentation generation
//    - Format checking and linting

// Try experimenting with these concepts:
// 1. Create variables of different numeric types and observe overflow behavior
// 2. Experiment with different compound types (tuples, arrays)
// 3. Write functions that demonstrate expression-based returns
// 4. Use different loop constructs and early returns
// 5. Explore the size_of and align_of functions for different types
