// Module 2: Ownership and Memory Management
// This module provides a deep dive into Rust's unique approach to memory management.
// We'll explore not just how ownership works, but why it exists and how it compares
// to other memory management approaches like garbage collection or manual management.

mod problems;

use std::mem;

fn main() {
    // Run the practice problems
    problems::run_exercises();

    // ===============================
    // 1. Memory Management in Rust
    // ===============================
    println!("\n1. Memory Management in Rust:");
    println!("---------------------------");
    // Rust's approach vs other languages:
    // - C/C++: Manual memory management (malloc/free, new/delete)
    // - Java/Go: Garbage collection (runtime overhead)
    // - Rust: Ownership system (compile-time checks, zero runtime cost)

    // Example: Memory layout and ownership
    #[derive(Debug)]
    struct Point {
        x: i32, // Stored inline (on stack)
        y: i32, // Stored inline (on stack)
    }

    #[derive(Debug)]
    struct Line {
        start: Point,    // Stored inline
        end: Box<Point>, // Stored on heap (Box)
    }

    let point = Point { x: 1, y: 2 };
    println!("Point size: {} bytes", mem::size_of::<Point>());
    println!("Point alignment: {} bytes", mem::align_of::<Point>());

    let line = Line {
        start: point,                        // Moves point into line.start
        end: Box::new(Point { x: 3, y: 4 }), // Allocates on heap
    };
    println!("Line: {:?}", line);
    // println!("Point: {:?}", point);  // Error: point was moved

    // ===============================
    // 2. Ownership Deep Dive
    // ===============================
    println!("\n2. Ownership Deep Dive:");
    println!("--------------------");

    // Understanding moves
    let text = String::from("Hello"); // text owns the heap memory
    let text2 = text; // Ownership moves to text2
                      // After move:
                      // - Original value is invalidated
                      // - No memory is copied (zero-cost abstraction)
                      // - Rust guarantees no use-after-move bugs

    // Clone vs Copy
    #[derive(Debug, Clone, Copy)] // Opt-in to Copy trait
    struct Number(i32); // Tuple struct

    #[derive(Debug, Clone)] // Clone but not Copy
    struct Text(String); // Contains heap data

    let num1 = Number(42);
    let num2 = num1; // Copies the value (cheap)
    println!("Both numbers: {:?} {:?}", num1, num2);

    let text1 = Text(String::from("Hello"));
    let text2 = text1.clone(); // Explicitly clone (potentially expensive)
    println!("Both texts: {:?} {:?}", text1, text2);

    // ===============================
    // 3. Advanced Borrowing Patterns
    // ===============================
    println!("\n3. Advanced Borrowing Patterns:");
    println!("----------------------------");

    // Self-referential structs (usually need lifetime annotations)
    // Fixed self-referential struct with proper lifetime
    struct Parser<'a> {
        content: String,
        current_pos: &'a str, // References part of content
    }

    impl<'a> Parser<'a> {
        // This function now takes a reference to avoid the self-referential issue
        fn new(content: String) -> (Parser<'a>, &'a str) {
            let content_ref = Box::leak(Box::new(content));
            let current_pos = &content_ref[..];
            (
                Parser {
                    content: String::new(), // Placeholder, actual content is leaked
                    current_pos,
                },
                current_pos,
            )
        }
    }

    // Alternative approach using separate allocation
    struct SafeParser {
        content: String,
        current_pos_idx: usize,
    }

    impl SafeParser {
        fn new(content: String) -> Self {
            SafeParser {
                content,
                current_pos_idx: 0,
            }
        }

        fn current_pos(&self) -> &str {
            &self.content[self.current_pos_idx..]
        }
    }

    // Multiple mutable borrows (non-overlapping)
    let mut numbers = vec![1, 2, 3, 4, 5];

    // Safe because borrows don't overlap
    let (left, right) = numbers.split_at_mut(3);
    left[0] = 10;
    right[0] = 20;
    println!("Modified vector: {:?}", numbers);

    // ===============================
    // 4. Lifetimes in Detail
    // ===============================
    println!("\n4. Lifetimes in Detail:");
    println!("---------------------");

    // Explicit lifetime annotations
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // Lifetime elision rules
    fn first_word(s: &str) -> &str {
        // Lifetimes elided
        match s.find(' ') {
            Some(pos) => &s[..pos],
            None => s,
        }
    }

    // Static lifetime
    static PROGRAM_NAME: &str = "Rust Demo"; // 'static lifetime
    println!("Program: {}", PROGRAM_NAME);

    // ===============================
    // 5. Memory Safety Guarantees
    // ===============================
    println!("\n5. Memory Safety Guarantees:");
    println!("--------------------------");

    // Example: Preventing data races
    let mut data = vec![1, 2, 3];

    // This would cause a data race in other languages
    // let reference1 = &mut data;
    // let reference2 = &mut data;  // Error: second mutable borrow

    // Safe concurrent modification
    {
        let reference = &mut data;
        reference.push(4);
    } // mutable borrow ends here

    data.push(5); // OK because previous borrow ended

    // Example: Preventing use after free
    let reference: Option<&String>;
    {
        let value = String::from("temporary");
        // reference = &value;  // Error: would create dangling reference
    } // value is dropped here

    // ===============================
    // 6. Zero-Cost Abstractions
    // ===============================
    println!("\n6. Zero-Cost Abstractions:");
    println!("--------------------------");

    // Example: Iterator vs manual indexing
    let vec = vec![1, 2, 3, 4, 5];

    // These compile to the same efficient machine code
    let sum1: i32 = vec.iter().sum();
    let mut sum2 = 0;
    for i in 0..vec.len() {
        sum2 += vec[i];
    }
    println!("Sums: {} {}", sum1, sum2);

    demonstrate_advanced_patterns();
}

// Advanced ownership patterns demonstration
fn demonstrate_advanced_patterns() {
    // ===============================
    // 7. Advanced Patterns
    // ===============================
    println!("\n7. Advanced Patterns:");
    println!("------------------");

    // Temporary ownership transfer
    let mut vec = vec![1, 2, 3];

    // Takes ownership temporarily and returns it
    let vec = take_and_return(vec);
    println!("Got vector back: {:?}", vec);

    // Scoped threads (ownership across threads)
    let handle = std::thread::spawn(|| {
        // This closure takes ownership of its captured variables
        let data = vec![1, 2, 3];
        println!("Processing in thread: {:?}", data);
    });
    handle.join().unwrap();

    // RAII (Resource Acquisition Is Initialization)
    struct ResourceGuard {
        data: String,
    }

    impl Drop for ResourceGuard {
        fn drop(&mut self) {
            println!("Cleaning up resource: {}", self.data);
        }
    }

    {
        let _guard = ResourceGuard {
            data: String::from("important data"),
        };
        // Resource is automatically cleaned up when guard goes out of scope
    }
}

// Helper functions demonstrating ownership patterns
fn take_and_return(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(4);
    vec // Return ownership
}

// Notes on Rust's Memory Safety:
//
// 1. Compile-Time Guarantees
//    - No null pointer dereferences
//    - No dangling pointers
//    - No buffer overflows
//    - No data races
//    - No use-after-free
//
// 2. Zero Runtime Cost
//    - No garbage collector
//    - No reference counting (unless explicitly used)
//    - No runtime bounds checking (except array access)
//
// 3. Predictable Performance
//    - Deterministic resource cleanup
//    - No stop-the-world pauses
//    - No memory fragmentation
//
// 4. Memory Model
//    - Stack: Fast, automatic cleanup, fixed size
//    - Heap: Flexible size, ownership tracked
//    - No garbage collector overhead
//
// 5. Common Patterns
//    - RAII for resource management
//    - Scope-based cleanup
//    - Move semantics for ownership transfer
//    - Borrowing for temporary access

// Try experimenting with these concepts:
// 1. Create self-referential structures
// 2. Implement custom Drop traits
// 3. Use scoped threads with ownership
// 4. Create zero-cost abstractions
// 5. Explore lifetime annotations
