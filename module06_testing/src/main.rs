// Module 6: Testing and Documentation
// This module explores Rust's testing and documentation features:
// - Test organization patterns
// - Unit tests vs integration tests
// - Test doubles (mocks, stubs)
// - Property-based testing
// - Documentation tests
// - API documentation best practices
// - Benchmarking and criterion.rs
// - Continuous Integration setup

mod problems;

use std::fmt;

fn main() {
    // Run the practice problems
    problems::run_exercises();

    // ===============================
    // 1. Test Organization Patterns
    // ===============================
    println!("\n1. Test Organization Patterns:");
    println!("---------------------------");
    // Rust provides multiple ways to organize tests:
    // - Unit tests: In the same file as the code, in a tests module
    // - Integration tests: In the tests/ directory
    // - Documentation tests: In the documentation comments

    // Unit test example
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_addition() {
            assert_eq!(2 + 2, 4);
        }

        #[test]
        fn test_string_equality() {
            assert_eq!("hello".to_string(), "hello");
        }
    }

    println!("Unit tests are defined in a #[cfg(test)] module");
    println!("Integration tests are placed in the tests/ directory");
    println!("Doc tests are written in documentation comments");

    // ===============================
    // 2. Test Attributes and Macros
    // ===============================
    println!("\n2. Test Attributes and Macros:");
    println!("---------------------------");
    // Rust provides various attributes and macros for testing:
    // - #[test]: Marks a function as a test
    // - #[should_panic]: Test should panic
    // - #[ignore]: Skip this test
    // - assert!, assert_eq!, assert_ne!: Assertion macros

    println!("Test attributes:");
    println!("  #[test] - Mark a function as a test");
    println!("  #[should_panic] - Test should panic");
    println!("  #[ignore] - Skip this test");
    println!("  #[bench] - Mark a function as a benchmark (nightly only)");

    println!("\nAssertion macros:");
    println!("  assert!(expression) - Assert expression is true");
    println!("  assert_eq!(left, right) - Assert equality");
    println!("  assert_ne!(left, right) - Assert inequality");

    // ===============================
    // 3. Test Doubles
    // ===============================
    println!("\n3. Test Doubles:");
    println!("---------------");
    // Test doubles are replacements for dependencies in tests:
    // - Mocks: Objects pre-programmed with expectations
    // - Stubs: Provide canned answers to calls
    // - Fakes: Working implementations but not suitable for production
    // - Spies: Record calls they receive

    // Example: Database interface
    trait Database {
        fn get_user(&self, id: u64) -> Option<User>;
        fn save_user(&mut self, user: &User) -> bool;
    }

    // User struct for demonstration
    struct User {
        id: u64,
        name: String,
    }

    // Mock implementation for testing
    struct MockDatabase {
        users: Vec<User>,
    }

    impl Database for MockDatabase {
        fn get_user(&self, id: u64) -> Option<User> {
            self.users.iter().find(|u| u.id == id).map(|u| User {
                id: u.id,
                name: u.name.clone(),
            })
        }

        fn save_user(&mut self, user: &User) -> bool {
            // Remove existing user with same id
            self.users.retain(|u| u.id != user.id);
            // Add new user
            self.users.push(User {
                id: user.id,
                name: user.name.clone(),
            });
            true
        }
    }

    println!("Test doubles allow testing code in isolation");
    println!("Rust's trait system makes it easy to create mock implementations");

    // ===============================
    // 4. Property-Based Testing
    // ===============================
    println!("\n4. Property-Based Testing:");
    println!("------------------------");
    // Property-based testing generates random inputs to test properties
    // that should hold for all inputs
    // - proptest: Popular property testing library
    // - quickcheck: Another property testing library

    println!("Property-based testing tests properties that should hold for all inputs");
    println!("Example property: reverse(reverse(list)) == list");
    println!("Libraries like proptest and quickcheck generate random inputs");

    // Example property (pseudocode)
    // #[test]
    // fn test_reverse_property() {
    //     proptest!(|(list: Vec<i32>)| {
    //         let reversed = reverse(&list);
    //         let double_reversed = reverse(&reversed);
    //         assert_eq!(list, double_reversed);
    //     });
    // }

    // ===============================
    // 5. Documentation Tests
    // ===============================
    println!("\n5. Documentation Tests:");
    println!("---------------------");
    // Documentation tests serve dual purposes:
    // - Ensure examples in documentation work
    // - Provide additional test coverage

    // Example documentation with tests
    /// Adds two numbers together.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = my_crate::add(2, 3);
    /// assert_eq!(result, 5);
    /// ```
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    println!("Documentation tests ensure examples in docs work");
    println!("They run with `cargo test` and appear in generated docs");
    println!("They help keep documentation and code in sync");

    // ===============================
    // 6. API Documentation Best Practices
    // ===============================
    println!("\n6. API Documentation Best Practices:");
    println!("--------------------------------");
    // Rust has conventions for documentation:
    // - Start with a short summary
    // - Use sections: Examples, Panics, Errors, Safety, etc.
    // - Include examples that compile and run

    println!("Documentation conventions:");
    println!("  - Start with a short summary");
    println!("  - Use sections: Examples, Panics, Errors, Safety");
    println!("  - Include runnable examples");
    println!("  - Document panics and errors");
    println!("  - Use Markdown formatting");

    // Example of well-documented function
    /// Divides two numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = my_crate::divide(10.0, 2.0);
    /// assert_eq!(result, Ok(5.0));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if `divisor` is zero.
    ///
    /// ```
    /// let result = my_crate::divide(10.0, 0.0);
    /// assert!(result.is_err());
    /// ```
    fn divide(dividend: f64, divisor: f64) -> Result<f64, String> {
        if divisor == 0.0 {
            Err("Division by zero".to_string())
        } else {
            Ok(dividend / divisor)
        }
    }

    // ===============================
    // 7. Benchmarking
    // ===============================
    println!("\n7. Benchmarking:");
    println!("--------------");
    // Benchmarking measures code performance:
    // - Built-in benchmarking (nightly only)
    // - criterion.rs: Statistical benchmarking
    // - Microbenchmarks vs. real-world performance

    println!("Benchmarking options:");
    println!("  - Built-in benchmarking (nightly only)");
    println!("  - criterion.rs for statistical benchmarking");
    println!("  - Custom benchmarking with std::time::Instant");

    // Example benchmark (nightly only)
    // #[bench]
    // fn bench_add(b: &mut test::Bencher) {
    //     b.iter(|| add(2, 3));
    // }

    // Example with criterion.rs (pseudocode)
    // fn criterion_benchmark(c: &mut Criterion) {
    //     c.bench_function("add 2 + 3", |b| b.iter(|| add(2, 3)));
    // }

    // ===============================
    // 8. Continuous Integration
    // ===============================
    println!("\n8. Continuous Integration:");
    println!("------------------------");
    // CI ensures code quality across changes:
    // - GitHub Actions, Travis CI, CircleCI
    // - Running tests on multiple platforms
    // - Code coverage reporting
    // - Automated deployment

    println!("CI workflow typically includes:");
    println!("  - Running tests on multiple platforms");
    println!("  - Checking code formatting with rustfmt");
    println!("  - Running clippy for linting");
    println!("  - Measuring code coverage");
    println!("  - Building documentation");
}

// Notes on Testing and Documentation:
//
// 1. Testing Philosophy
//    - Tests as documentation
//    - Tests as regression prevention
//    - Tests as design tools
//    - Balance between unit and integration tests
//
// 2. Documentation Philosophy
//    - Documentation as a first-class citizen
//    - Executable examples ensure correctness
//    - Consistent style and organization
//    - Generated HTML documentation with cargo doc
//
// 3. Test Organization
//    - Unit tests: Close to the code they test
//    - Integration tests: Test public API
//    - Documentation tests: Ensure examples work
//    - Benchmark tests: Measure performance
//
// 4. Best Practices
//    - Write tests first (TDD) or alongside code
//    - Test edge cases and error conditions
//    - Use meaningful test names
//    - Keep tests fast and independent
//    - Document public API thoroughly
//
// 5. Tools
//    - cargo test: Run tests
//    - cargo doc: Generate documentation
//    - cargo bench: Run benchmarks (nightly)
//    - rustdoc: Documentation generator
//    - proptest/quickcheck: Property-based testing
//    - criterion.rs: Statistical benchmarking

// Try experimenting with these concepts:
// 1. Write unit tests with different assertion macros
// 2. Create integration tests in a tests/ directory
// 3. Write documentation with examples that run as tests
// 4. Create a mock implementation of a trait for testing
// 5. Set up a simple benchmark to measure performance
