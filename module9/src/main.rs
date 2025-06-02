// Module 9: Advanced Pattern Matching
// This module explores Rust's powerful pattern matching capabilities:
// - Pattern matching internals
// - Destructuring complex types
// - Match guards and bindings
// - Custom patterns with ref and ref mut
// - Ranges and multiple patterns
// - Pattern matching optimization

mod problems;

fn main() {
    // Run the practice problems
    problems::run_exercises();

    // ===============================
    // 1. Pattern Matching Internals
    // ===============================
    println!("\n1. Pattern Matching Internals:");
    println!("--------------------------");
    // Pattern matching is a core feature of Rust
    // - Compiler generates efficient code
    // - Exhaustiveness checking ensures all cases are handled
    // - Patterns can be nested and combined

    // Simple match expression
    let value = 42;
    match value {
        0 => println!("Zero"),
        1 => println!("One"),
        _ => println!("Something else: {}", value),
    }

    // How match works internally:
    println!("\nMatch internals:");
    println!("  - Compiler generates decision tree");
    println!("  - Optimized for the specific patterns");
    println!("  - Exhaustiveness checking ensures all cases are covered");
    println!("  - Can be as efficient as if/else chains or switch statements");

    // ===============================
    // 2. Destructuring Complex Types
    // ===============================
    println!("\n2. Destructuring Complex Types:");
    println!("----------------------------");
    // Destructuring allows extracting values from complex types
    // - Structs, enums, tuples, arrays, slices
    // - Nested patterns
    // - Partial destructuring

    // Struct destructuring
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 10, y: 20 };
    let Point { x, y } = point;
    println!("Destructured point: x={}, y={}", x, y);

    // Enum destructuring
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }

    // Nested destructuring
    let nested = (Point { x: 1, y: 2 }, Point { x: 3, y: 4 });
    let ((Point { x: x1, y: y1 }), (Point { x: x2, y: y2 })) = nested;
    println!("Nested points: ({}, {}), ({}, {})", x1, y1, x2, y2);

    // ===============================
    // 3. Match Guards and Bindings
    // ===============================
    println!("\n3. Match Guards and Bindings:");
    println!("-------------------------");
    // Match guards add conditions to patterns
    // Bindings allow capturing values while matching

    // Match guards
    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("Equal"),
        (x, y) if x + y == 0 => println!("Sum is zero"),
        (x, y) if x % 2 == 0 && y % 2 == 0 => println!("Both even"),
        _ => println!("No special case"),
    }

    // @ bindings
    let x = 5;
    match x {
        n @ 1..=5 => println!("Got a number in range 1-5: {}", n),
        n @ 6..=10 => println!("Got a number in range 6-10: {}", n),
        _ => println!("Number out of range"),
    }

    // Combining destructuring with @ bindings
    let msg = Message::ChangeColor(255, 0, 255);
    match msg {
        Message::ChangeColor(r @ 255, _, b) => {
            println!("Magenta-ish color with blue component: {}", b)
        }
        Message::ChangeColor(r, g, b) => println!("Other color: RGB({}, {}, {})", r, g, b),
        _ => println!("Not a color message"),
    }

    // ===============================
    // 4. Custom Patterns with ref and ref mut
    // ===============================
    println!("\n4. Custom Patterns with ref and ref mut:");
    println!("-----------------------------------");
    // ref and ref mut allow borrowing in patterns
    // - ref creates an immutable reference
    // - ref mut creates a mutable reference

    // ref pattern
    let value = 5;
    match value {
        ref r => println!("Got a reference to {}", r),
    }

    // ref mut pattern
    let mut value = 5;
    match value {
        ref mut r => {
            *r += 1;
            println!("Added 1 to value: {}", r);
        }
    }
    println!("Value after match: {}", value);

    // Combining ref with destructuring
    let mut point = Point { x: 10, y: 20 };
    match point {
        Point {
            x: ref mut rx,
            y: ref ry,
        } => {
            *rx += 1;
            println!("Modified point: x={}, y={}", rx, ry);
        }
    }
    println!("Point after match: x={}, y={}", point.x, point.y);

    // ===============================
    // 5. Ranges and Multiple Patterns
    // ===============================
    println!("\n5. Ranges and Multiple Patterns:");
    println!("----------------------------");
    // Ranges allow matching a range of values
    // Multiple patterns can be combined with |

    // Range patterns
    let value = 42;
    match value {
        0 => println!("Zero"),
        1..=9 => println!("Single digit"),
        10..=99 => println!("Double digits"),
        100..=999 => println!("Triple digits"),
        _ => println!("Very large number"),
    }

    // Multiple patterns with |
    let c = 'z';
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => println!("Vowel"),
        'a'..='z' => println!("Lowercase consonant"),
        'A'..='Z' => println!("Uppercase letter"),
        _ => println!("Something else"),
    }

    // Combining ranges and multiple patterns
    let value = 42;
    match value {
        0 | 1 => println!("Zero or one"),
        2..=10 | 20..=30 => println!("In range 2-10 or 20-30"),
        40..=50 => println!("In range 40-50"),
        _ => println!("Something else"),
    }

    // ===============================
    // 6. Irrefutable and Refutable Patterns
    // ===============================
    println!("\n6. Irrefutable and Refutable Patterns:");
    println!("----------------------------------");
    // Irrefutable patterns always match
    // Refutable patterns might not match

    // Irrefutable pattern in let
    let (a, b) = (1, 2); // Always matches
    println!("Irrefutable pattern: a={}, b={}", a, b);

    // Refutable pattern in if let
    if let Some(value) = Some(42) {
        println!("Refutable pattern matched: {}", value);
    }

    // Refutable pattern in while let
    let mut values = vec![1, 2, 3, 4, 5];
    while let Some(value) = values.pop() {
        println!("Popped value: {}", value);
    }

    // ===============================
    // 7. Pattern Matching Optimization
    // ===============================
    println!("\n7. Pattern Matching Optimization:");
    println!("-----------------------------");
    // Rust's pattern matching is optimized by the compiler
    // - Decision trees
    // - Jump tables
    // - Inlining

    // Example of optimized match (conceptual)
    let value = 42;
    match value {
        0..=9 => println!("Single digit"),
        10..=99 => println!("Double digits"),
        100..=999 => println!("Triple digits"),
        _ => println!("Very large number"),
    }

    println!("\nCompiler optimizations:");
    println!("  - Range checks compiled to efficient comparisons");
    println!("  - Small integer matches may use jump tables");
    println!("  - Complex patterns generate optimized decision trees");
    println!("  - Dead code elimination for unreachable patterns");

    // ===============================
    // 8. Advanced Applications
    // ===============================
    println!("\n8. Advanced Applications:");
    println!("----------------------");
    // Pattern matching enables powerful programming patterns
    // - State machines
    // - Visitor pattern
    // - Parsing and tokenization

    // State machine example
    #[derive(Debug)]
    enum State {
        Start,
        Processing { progress: f64 },
        Done { result: String },
        Error { message: String },
    }

    let state = State::Processing { progress: 0.5 };
    match state {
        State::Start => println!("In start state"),
        State::Processing { progress } if progress < 0.5 => {
            println!("Processing (early stage): {}%", progress * 100.0)
        }
        State::Processing { progress } => println!("Processing: {}%", progress * 100.0),
        State::Done { result } => println!("Done with result: {}", result),
        State::Error { message } => println!("Error: {}", message),
    }

    // Visitor pattern example (simplified)
    enum Expression {
        Number(i32),
        Add(Box<Expression>, Box<Expression>),
        Subtract(Box<Expression>, Box<Expression>),
    }

    fn evaluate(expr: &Expression) -> i32 {
        match expr {
            Expression::Number(n) => *n,
            Expression::Add(left, right) => evaluate(left) + evaluate(right),
            Expression::Subtract(left, right) => evaluate(left) - evaluate(right),
        }
    }

    let expr = Expression::Add(
        Box::new(Expression::Number(5)),
        Box::new(Expression::Subtract(
            Box::new(Expression::Number(10)),
            Box::new(Expression::Number(3)),
        )),
    );

    println!("Expression result: {}", evaluate(&expr)); // 5 + (10 - 3) = 12
}

// Notes on Advanced Pattern Matching:
//
// 1. Pattern Types
//    - Literals: 1, "hello", true
//    - Variables: x, y, z
//    - Wildcards: _
//    - Ranges: 1..=5, 'a'..='z'
//    - Multiple patterns: | operator
//    - Destructuring: tuples, structs, enums
//    - Guards: if conditions
//    - Bindings: @ operator
//
// 2. Pattern Contexts
//    - match expressions
//    - if let expressions
//    - while let expressions
//    - for loops
//    - let statements
//    - function parameters
//
// 3. Exhaustiveness Checking
//    - Compiler ensures all possibilities are covered
//    - Prevents bugs from missing cases
//    - Helps with code maintenance
//
// 4. Performance Characteristics
//    - Optimized by the compiler
//    - No runtime overhead for pattern matching
//    - Complex patterns compile to efficient code
//
// 5. Best Practices
//    - Use pattern matching for clarity
//    - Prefer destructuring over field access
//    - Use guards for complex conditions
//    - Leverage exhaustiveness for safer code
//    - Consider readability vs. complexity

// Try experimenting with these concepts:
// 1. Create complex nested patterns
// 2. Use pattern matching for error handling
// 3. Implement a state machine with pattern matching
// 4. Explore optimizations for different pattern types
// 5. Use pattern matching for parsing and tokenization
