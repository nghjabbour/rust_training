// Module 1: Practice Problems
// These exercises are designed to help you understand Rust's fundamentals
// through hands-on practice. Each problem includes detailed explanations
// of the underlying concepts and why Rust works the way it does.

pub fn run_exercises() {
    println!("Module 1 Exercises - Rust Fundamentals");
    println!("===================================\n");

    exercise1();
    exercise2();
    exercise3();
    exercise4();
    exercise5();
}

// Exercise 1: Type System and Memory Layout
// This exercise demonstrates:
// - How Rust's type system ensures memory safety
// - Stack vs heap allocation
// - Size and alignment of different types
fn exercise1() {
    println!("Exercise 1: Type System and Memory Layout");
    println!("------------------------------------");
    println!("TODO: Implement the analyze_types function\n");

    // Function to analyze and print type information
    fn analyze_types<T: std::fmt::Debug>(value: T) {
        // Get the type name (requires nightly Rust for better output)
        println!("Type: {:?}", std::any::type_name::<T>());

        // Print size and alignment
        println!("Size: {} bytes", std::mem::size_of::<T>());
        println!("Alignment: {} bytes", std::mem::align_of::<T>());

        // Print whether the type implements various traits
        println!("Implements Copy: {}", is_copy::<T>());
        println!("Implements Clone: {}", is_clone::<T>());
        println!("Implements Send: {}", is_send::<T>());
        println!("Implements Sync: {}", is_sync::<T>());
    }

    // Helper functions to check trait implementations
    fn is_copy<T: ?Sized>() -> bool {
        // needs_drop returns the opposite of what we want
        // A type is Copy if it doesn't need drop
        !std::mem::needs_drop::<T>()
    }

    fn is_clone<T: ?Sized>() -> bool {
        trait IsClone {
            fn is_clone(&self) -> bool {
                true
            }
        }
        impl<T: Clone> IsClone for T {}
        true
    }

    fn is_send<T: ?Sized>() -> bool {
        trait IsSend {
            fn is_send(&self) -> bool {
                true
            }
        }
        impl<T: Send> IsSend for T {}
        true
    }

    fn is_sync<T: ?Sized>() -> bool {
        trait IsSync {
            fn is_sync(&self) -> bool {
                true
            }
        }
        impl<T: Sync> IsSync for T {}
        true
    }

    // Test various types
    println!("Analyzing i32:");
    analyze_types(42_i32);

    println!("\nAnalyzing String:");
    analyze_types(String::from("hello"));

    println!("\nAnalyzing custom struct:");
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    analyze_types(Point { x: 0, y: 0 });
}

// Exercise 2: Expression Evaluation
// This exercise demonstrates:
// - Rust's expression-based nature
// - Operator precedence
// - Type inference
// - Control flow as expressions
fn exercise2() {
    println!("\nExercise 2: Expression Evaluation");
    println!("------------------------------");
    println!("TODO: Implement the evaluate_expressions function\n");

    // Function that demonstrates Rust's expression-based nature
    fn evaluate_expressions() -> i32 {
        let x = 5;
        let y = 10;

        // Block expression
        let block_result = {
            let temp = x * 2;
            temp + y // Note: no semicolon means this is the block's value
        };

        // If expression
        let if_result = if x > y { x } else { y };

        // Match expression
        let match_result = match x {
            0 => "zero",
            1 => "one",
            _ => "other",
        }
        .len(); // We can call methods on the result

        // Loop expression with break value
        let loop_result = loop {
            if x < y {
                break x + y;
            }
        };

        // The final expression determines the function's return value
        block_result + if_result + (match_result as i32) + loop_result
    }

    let result = evaluate_expressions();
    println!("Expression evaluation result: {}", result);
}

// Exercise 3: Memory Management
// This exercise demonstrates:
// - Stack vs heap allocation
// - Move semantics
// - Borrowing rules
// - Lifetimes (implicitly)
fn exercise3() {
    println!("\nExercise 3: Memory Management");
    println!("--------------------------");
    println!("TODO: Implement the memory_examples function\n");

    // Struct that owns heap memory
    struct HeapData {
        value: Box<i32>,
        text: String,
    }

    // Struct that only uses stack memory
    #[derive(Copy, Clone)]
    struct StackData {
        value: i32,
        flag: bool,
    }

    // Function that demonstrates memory management
    fn memory_examples() {
        // Stack allocation
        let stack_data = StackData {
            value: 42,
            flag: true,
        };

        // Heap allocation
        let heap_data = HeapData {
            value: Box::new(42),
            text: String::from("hello"),
        };

        // Move semantics
        let moved_stack = stack_data; // Creates a copy (implements Copy)
        println!("Original still available: {}", stack_data.value);

        let moved_heap = heap_data; // Moves ownership
                                    // println!("Original no longer available: {}", heap_data.value); // Would not compile

        // Borrowing
        let borrowed = &moved_heap; // Immutable borrow
        println!("Borrowed value: {}", borrowed.value);

        // Multiple borrows
        let ref1 = &moved_stack;
        let ref2 = &moved_stack; // Multiple immutable borrows are okay
        println!("Multiple refs: {} {}", ref1.value, ref2.value);

        // Mutable borrowing
        let mut mutable = StackData {
            value: 10,
            flag: false,
        };
        let mut_ref = &mut mutable; // Only one mutable borrow allowed at a time
        mut_ref.value += 1;
        println!("Modified value: {}", mut_ref.value);
    }

    memory_examples();
}

// Exercise 4: Control Flow and Pattern Matching
// This exercise demonstrates:
// - Pattern matching capabilities
// - Match expressions
// - If let and while let
// - Range patterns
fn exercise4() {
    println!("\nExercise 4: Control Flow and Pattern Matching");
    println!("----------------------------------------");
    println!("TODO: Implement the pattern_matching function\n");

    // Enum for demonstration
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // Function that demonstrates pattern matching
    fn pattern_matching() {
        let messages = vec![
            Message::Quit,
            Message::Move { x: 3, y: 4 },
            Message::Write(String::from("hello")),
            Message::ChangeColor(255, 0, 0),
        ];

        for msg in messages {
            match msg {
                Message::Quit => println!("Quit message received"),
                Message::Move { x, y } => println!("Move to ({}, {})", x, y),
                Message::Write(text) => println!("Text message: {}", text),
                Message::ChangeColor(r, g, b) => {
                    println!("Change color to RGB({}, {}, {})", r, g, b)
                }
            }
        }

        // Range patterns
        let number = 42;
        match number {
            0 => println!("Zero"),
            1..=9 => println!("Single digit"),
            10..=99 => println!("Double digits"),
            _ => println!("Large number"),
        }

        // Match guards
        let pair = (2, -2);
        match pair {
            (x, y) if x == y => println!("Equal"),
            (x, y) if x + y == 0 => println!("Sum is zero"),
            _ => println!("No special case"),
        }
    }

    pattern_matching();
}

// Exercise 5: Functions and Closures
// This exercise demonstrates:
// - Function pointers
// - Closures and their environment
// - Higher-order functions
// - Type inference with closures
fn exercise5() {
    println!("\nExercise 5: Functions and Closures");
    println!("-------------------------------");
    println!("TODO: Implement the higher_order_functions example\n");

    // Function that takes a function pointer
    fn apply_function(f: fn(i32) -> i32, x: i32) -> i32 {
        f(x)
    }

    // Function that returns a closure
    fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
        move |x| x * factor
    }

    // Higher-order function example
    fn higher_order_functions() {
        // Function pointer
        fn square(x: i32) -> i32 {
            x * x
        }
        println!("Square of 5: {}", apply_function(square, 5));

        // Closure with environment
        let factor = 3;
        let multiplier = create_multiplier(factor);
        println!("5 times {}: {}", factor, multiplier(5));

        // Iterator adaptors with closures
        let numbers = vec![1, 2, 3, 4, 5];
        let sum: i32 = numbers
            .iter()
            .map(|x| x * x) // Square each number
            .filter(|x| x % 2 == 0) // Keep only even numbers
            .sum(); // Sum the results
        println!("Sum of even squares: {}", sum);
    }

    higher_order_functions();
}

/* Example Solutions (Try to solve the exercises before looking at these!)

// Exercise 1 Solution:
fn analyze_types() {
    // Implementation details in the exercise
    // Focus on understanding the output and why different types
    // have different characteristics
}

// Exercise 2 Solution:
fn evaluate_expressions() -> i32 {
    let x = 5;
    let y = 10;

    let block_result = {
        let temp = x * 2;
        temp + y
    };

    let if_result = if x > y { x } else { y };

    let match_result = match x {
        0 => "zero",
        1 => "one",
        _ => "other",
    }.len();

    let loop_result = loop {
        if x < y {
            break x + y;
        }
    };

    block_result + if_result + match_result + loop_result
}

// Exercise 3 Solution:
fn memory_examples() {
    // Implementation showing proper memory management
    // Focus on understanding ownership transfers and borrowing rules
}

// Exercise 4 Solution:
fn pattern_matching() {
    // Implementation showing various pattern matching techniques
    // Focus on exhaustiveness and match guards
}

// Exercise 5 Solution:
fn higher_order_functions() {
    fn square(x: i32) -> i32 { x * x }
    println!("Square of 5: {}", apply_function(square, 5));

    let multiplier = create_multiplier(3);
    println!("5 times 3: {}", multiplier(5));

    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter()
        .map(|x| x * x)
        .filter(|x| x % 2 == 0)
        .sum();
    println!("Sum of even squares: {}", sum);
}
*/
