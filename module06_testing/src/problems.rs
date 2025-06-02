// Module 6: Practice Problems
// These exercises focus on testing and documentation in Rust.
// Each problem includes detailed explanations and examples to help you
// understand best practices for testing and documenting Rust code.

pub fn run_exercises() {
    println!("Module 6 Exercises - Testing and Documentation");
    println!("=========================================\n");

    exercise1();
    exercise2();
    exercise3();
    exercise4();
    exercise5();
}

// Exercise 1: Unit Testing
// This exercise demonstrates:
// - Writing effective unit tests
// - Using different assertion macros
// - Test organization patterns
fn exercise1() {
    println!("Exercise 1: Unit Testing");
    println!("--------------------");
    println!("TODO: Implement the Calculator struct and its tests\n");

    // A simple calculator with basic operations
    struct Calculator;

    impl Calculator {
        fn new() -> Self {
            Calculator
        }

        fn add(&self, a: i32, b: i32) -> i32 {
            unimplemented!("Implement the add method")
        }

        fn subtract(&self, a: i32, b: i32) -> i32 {
            unimplemented!("Implement the subtract method")
        }

        fn multiply(&self, a: i32, b: i32) -> i32 {
            unimplemented!("Implement the multiply method")
        }

        fn divide(&self, a: i32, b: i32) -> Result<i32, String> {
            unimplemented!("Implement the divide method")
        }
    }

    // Unit tests for the Calculator
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_add() {
            unimplemented!("Write a test for the add method");
        }

        #[test]
        fn test_subtract() {
            unimplemented!("Write a test for the subtract method");
        }

        #[test]
        fn test_multiply() {
            unimplemented!("Write a test for the multiply method");
        }

        #[test]
        fn test_divide_valid() {
            unimplemented!("Write a test for valid division");
        }

        #[test]
        fn test_divide_by_zero() {
            unimplemented!("Write a test for division by zero");
        }
    }

    // Note: To run these tests, use:
    // cargo test --package module6
    println!("Unit tests have been defined for the Calculator struct.");
    println!("Run them with: cargo test --package module6");
}

// Exercise 2: Test Doubles
// This exercise demonstrates:
// - Creating mock objects for testing
// - Dependency injection
// - Testing code with external dependencies
fn exercise2() {
    println!("\nExercise 2: Test Doubles");
    println!("--------------------");
    println!("TODO: Implement the UserService and its tests\n");

    // User struct
    struct User {
        id: u64,
        name: String,
        email: String,
    }

    // Database interface
    trait UserDatabase {
        fn get_user(&self, id: u64) -> Option<User>;
        fn save_user(&mut self, user: User) -> Result<(), String>;
        fn delete_user(&mut self, id: u64) -> Result<(), String>;
    }

    // Service that uses the database
    struct UserService<T: UserDatabase> {
        database: T,
    }

    impl<T: UserDatabase> UserService<T> {
        fn new(database: T) -> Self {
            UserService { database }
        }

        fn get_user_name(&self, id: u64) -> Option<String> {
            unimplemented!("Implement get_user_name")
        }

        fn register_user(&mut self, name: String, email: String) -> Result<u64, String> {
            unimplemented!("Implement register_user")
        }

        fn delete_user(&mut self, id: u64) -> Result<(), String> {
            unimplemented!("Implement delete_user")
        }
    }

    // Mock database for testing
    #[cfg(test)]
    mod tests {
        use super::*;

        struct MockUserDatabase {
            users: Vec<User>,
        }

        impl MockUserDatabase {
            fn new() -> Self {
                unimplemented!("Implement MockUserDatabase::new")
            }
        }

        impl UserDatabase for MockUserDatabase {
            fn get_user(&self, id: u64) -> Option<User> {
                unimplemented!("Implement get_user for MockUserDatabase")
            }

            fn save_user(&mut self, user: User) -> Result<(), String> {
                unimplemented!("Implement save_user for MockUserDatabase")
            }

            fn delete_user(&mut self, id: u64) -> Result<(), String> {
                unimplemented!("Implement delete_user for MockUserDatabase")
            }
        }

        #[test]
        fn test_get_user_name() {
            unimplemented!("Write a test for get_user_name");
        }

        #[test]
        fn test_register_user() {
            unimplemented!("Write a test for register_user");
        }

        #[test]
        fn test_delete_user() {
            unimplemented!("Write a test for delete_user");
        }
    }

    println!("Test doubles have been defined for the UserService.");
    println!("Run the tests with: cargo test --package module6");
}

// Exercise 3: Documentation Tests
// This exercise demonstrates:
// - Writing effective documentation
// - Including runnable examples in documentation
// - Documentation best practices
fn exercise3() {
    println!("\nExercise 3: Documentation Tests");
    println!("---------------------------");
    println!("TODO: Document the StringUtils module\n");

    // StringUtils module with various string manipulation functions
    mod string_utils {
        /// Converts a string to title case.
        ///
        /// TODO: Add documentation with examples
        pub fn to_title_case(s: &str) -> String {
            unimplemented!("Implement to_title_case")
        }

        /// Truncates a string to the specified length.
        ///
        /// TODO: Add documentation with examples
        pub fn truncate(s: &str, max_length: usize) -> String {
            unimplemented!("Implement truncate")
        }

        /// Counts the number of words in a string.
        ///
        /// TODO: Add documentation with examples
        pub fn word_count(s: &str) -> usize {
            unimplemented!("Implement word_count")
        }

        /// Checks if a string is a palindrome.
        ///
        /// TODO: Add documentation with examples
        pub fn is_palindrome(s: &str) -> bool {
            unimplemented!("Implement is_palindrome")
        }
    }

    println!("Documentation has been added to the StringUtils module.");
    println!("Run the doc tests with: cargo test --doc");
}

// Exercise 4: Integration Testing
// This exercise demonstrates:
// - Setting up integration tests
// - Testing public API
// - Organizing test files
fn exercise4() {
    println!("\nExercise 4: Integration Testing");
    println!("--------------------------");
    println!("TODO: Create integration tests for the Config module\n");

    // Config module for loading and parsing configuration
    pub mod config {
        use std::collections::HashMap;
        use std::fs;
        use std::path::Path;

        /// Configuration structure
        pub struct Config {
            values: HashMap<String, String>,
        }

        impl Config {
            /// Creates a new empty configuration
            pub fn new() -> Self {
                Config {
                    values: HashMap::new(),
                }
            }

            /// Loads configuration from a file
            pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
                unimplemented!("Implement load_from_file")
            }

            /// Gets a configuration value
            pub fn get(&self, key: &str) -> Option<&String> {
                unimplemented!("Implement get")
            }

            /// Sets a configuration value
            pub fn set(&mut self, key: String, value: String) {
                unimplemented!("Implement set")
            }

            /// Saves configuration to a file
            pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
                unimplemented!("Implement save_to_file")
            }
        }
    }

    println!("The Config module has been defined.");
    println!("Create integration tests in the tests/ directory.");
    println!("Example test file structure:");
    println!("  tests/");
    println!("    config_tests.rs");
}

// Exercise 5: Property-Based Testing
// This exercise demonstrates:
// - Writing property-based tests
// - Defining properties that should hold
// - Testing with randomly generated inputs
fn exercise5() {
    println!("\nExercise 5: Property-Based Testing");
    println!("------------------------------");
    println!("TODO: Implement property-based tests for the sorting module\n");

    // Sorting module with various sorting algorithms
    mod sorting {
        /// Sorts a vector using bubble sort
        pub fn bubble_sort<T: Ord + Clone>(slice: &[T]) -> Vec<T> {
            unimplemented!("Implement bubble_sort")
        }

        /// Sorts a vector using insertion sort
        pub fn insertion_sort<T: Ord + Clone>(slice: &[T]) -> Vec<T> {
            unimplemented!("Implement insertion_sort")
        }

        /// Sorts a vector using quicksort
        pub fn quick_sort<T: Ord + Clone>(slice: &[T]) -> Vec<T> {
            unimplemented!("Implement quick_sort")
        }

        /// Checks if a slice is sorted
        pub fn is_sorted<T: Ord>(slice: &[T]) -> bool {
            unimplemented!("Implement is_sorted")
        }
    }

    // Property-based tests
    // Note: These would typically use a library like proptest or quickcheck
    #[cfg(test)]
    mod tests {
        use super::sorting::*;

        // Example property test (pseudocode)
        // #[test]
        // fn test_sort_idempotence() {
        //     // Property: Sorting an already sorted list should not change it
        //     proptest!(|(list: Vec<i32>)| {
        //         let sorted = bubble_sort(&list);
        //         let double_sorted = bubble_sort(&sorted);
        //         assert_eq!(sorted, double_sorted);
        //     });
        // }

        // TODO: Implement more property tests
    }

    println!("The sorting module has been defined.");
    println!("To implement property-based tests, add proptest to your Cargo.toml:");
    println!("  [dev-dependencies]");
    println!("  proptest = \"1.0\"");
}

/* Example Solutions (Try to solve the exercises before looking at these!)

// Exercise 1 Solution:
impl Calculator {
    fn new() -> Self {
        Calculator
    }

    fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b
    }

    fn multiply(&self, a: i32, b: i32) -> i32 {
        a * b
    }

    fn divide(&self, a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err("Division by zero".to_string())
        } else {
            Ok(a / b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let calc = Calculator::new();
        assert_eq!(calc.add(2, 3), 5);
        assert_eq!(calc.add(-2, 3), 1);
        assert_eq!(calc.add(0, 0), 0);
    }

    #[test]
    fn test_subtract() {
        let calc = Calculator::new();
        assert_eq!(calc.subtract(5, 3), 2);
        assert_eq!(calc.subtract(3, 5), -2);
        assert_eq!(calc.subtract(0, 0), 0);
    }

    #[test]
    fn test_multiply() {
        let calc = Calculator::new();
        assert_eq!(calc.multiply(2, 3), 6);
        assert_eq!(calc.multiply(-2, 3), -6);
        assert_eq!(calc.multiply(0, 5), 0);
    }

    #[test]
    fn test_divide_valid() {
        let calc = Calculator::new();
        assert_eq!(calc.divide(6, 3), Ok(2));
        assert_eq!(calc.divide(5, 2), Ok(2)); // Integer division
        assert_eq!(calc.divide(0, 5), Ok(0));
    }

    #[test]
    fn test_divide_by_zero() {
        let calc = Calculator::new();
        assert!(calc.divide(6, 0).is_err());
    }
}

// Exercise 2 Solution:
impl<T: UserDatabase> UserService<T> {
    fn get_user_name(&self, id: u64) -> Option<String> {
        self.database.get_user(id).map(|user| user.name)
    }

    fn register_user(&mut self, name: String, email: String) -> Result<u64, String> {
        let id = generate_id(); // In a real implementation, this would be more sophisticated
        let user = User { id, name, email };
        self.database.save_user(user)?;
        Ok(id)
    }

    fn delete_user(&mut self, id: u64) -> Result<(), String> {
        self.database.delete_user(id)
    }
}

fn generate_id() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod tests {
    impl MockUserDatabase {
        fn new() -> Self {
            MockUserDatabase { users: Vec::new() }
        }
    }

    impl UserDatabase for MockUserDatabase {
        fn get_user(&self, id: u64) -> Option<User> {
            self.users.iter().find(|u| u.id == id).cloned()
        }

        fn save_user(&mut self, user: User) -> Result<(), String> {
            // Check if user with same ID already exists
            if self.users.iter().any(|u| u.id == user.id) {
                return Err("User already exists".to_string());
            }
            self.users.push(user);
            Ok(())
        }

        fn delete_user(&mut self, id: u64) -> Result<(), String> {
            let initial_len = self.users.len();
            self.users.retain(|u| u.id != id);
            if self.users.len() == initial_len {
                Err("User not found".to_string())
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn test_get_user_name() {
        let mut db = MockUserDatabase::new();
        db.users.push(User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        });

        let service = UserService::new(db);
        assert_eq!(service.get_user_name(1), Some("Alice".to_string()));
        assert_eq!(service.get_user_name(2), None);
    }

    #[test]
    fn test_register_user() {
        let db = MockUserDatabase::new();
        let mut service = UserService::new(db);

        let result = service.register_user(
            "Bob".to_string(),
            "bob@example.com".to_string()
        );

        assert!(result.is_ok());
        let id = result.unwrap();
        assert_eq!(service.get_user_name(id), Some("Bob".to_string()));
    }

    #[test]
    fn test_delete_user() {
        let mut db = MockUserDatabase::new();
        db.users.push(User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        });

        let mut service = UserService::new(db);
        assert!(service.delete_user(1).is_ok());
        assert_eq!(service.get_user_name(1), None);
        assert!(service.delete_user(2).is_err());
    }
}

// Exercise 3 Solution:
/// Converts a string to title case.
///
/// Title case means the first letter of each word is capitalized,
/// while the rest are lowercase.
///
/// # Examples
///
/// ```
/// use module6::string_utils::to_title_case;
///
/// let result = to_title_case("hello world");
/// assert_eq!(result, "Hello World");
///
/// let result = to_title_case("RUST PROGRAMMING");
/// assert_eq!(result, "Rust Programming");
/// ```
pub fn to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let rest: String = chars.map(|c| c.to_lowercase().to_string()).collect();
                    format!("{}{}", first.to_uppercase(), rest)
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

/// Truncates a string to the specified length.
///
/// If the string is longer than `max_length`, it will be truncated and
/// an ellipsis ("...") will be appended. The total length including the
/// ellipsis will not exceed `max_length`.
///
/// # Examples
///
/// ```
/// use module6::string_utils::truncate;
///
/// let result = truncate("Hello, world!", 8);
/// assert_eq!(result, "Hello...");
///
/// let result = truncate("Short", 10);
/// assert_eq!(result, "Short");
/// ```
///
/// # Panics
///
/// Panics if `max_length` is less than 3, as that's not enough space for the ellipsis.
pub fn truncate(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        if max_length < 3 {
            panic!("max_length must be at least 3 to accommodate ellipsis");
        }
        format!("{}...", &s[0..max_length - 3])
    }
}

// Exercise 4 Solution:
// In src/lib.rs or src/config.rs:
impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        let mut config = Config::new();
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid line format: {}", line));
            }

            let key = parts[0].trim().to_string();
            let value = parts[1].trim().to_string();
            config.set(key, value);
        }

        Ok(config)
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }

    pub fn set(&mut self, key: String, value: String) {
        self.values.insert(key, value);
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let mut content = String::new();
        for (key, value) in &self.values {
            content.push_str(&format!("{}={}\n", key, value));
        }

        fs::write(path, content)
            .map_err(|e| format!("Failed to write file: {}", e))
    }
}

// In tests/config_tests.rs:
use module6::config::Config;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn test_load_and_save() {
    let file = NamedTempFile::new().unwrap();
    let path = file.path();

    fs::write(path, "key1=value1\nkey2=value2\n").unwrap();

    let config = Config::load_from_file(path).unwrap();
    assert_eq!(config.get("key1"), Some(&"value1".to_string()));
    assert_eq!(config.get("key2"), Some(&"value2".to_string()));

    let new_file = NamedTempFile::new().unwrap();
    let new_path = new_file.path();

    config.save_to_file(new_path).unwrap();
    let content = fs::read_to_string(new_path).unwrap();

    assert!(content.contains("key1=value1"));
    assert!(content.contains("key2=value2"));
}

// Exercise 5 Solution:
pub fn bubble_sort<T: Ord + Clone>(slice: &[T]) -> Vec<T> {
    let mut result = slice.to_vec();
    let n = result.len();

    for i in 0..n {
        for j in 0..n - i - 1 {
            if result[j] > result[j + 1] {
                result.swap(j, j + 1);
            }
        }
    }

    result
}

pub fn is_sorted<T: Ord>(slice: &[T]) -> bool {
    slice.windows(2).all(|w| w[0] <= w[1])
}

// In tests with proptest:
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_sort_idempotence(ref list in prop::collection::vec(0..100i32, 0..100)) {
        let sorted = bubble_sort(list);
        let double_sorted = bubble_sort(&sorted);
        prop_assert_eq!(sorted, double_sorted);
    }

    #[test]
    fn test_sort_result_is_sorted(ref list in prop::collection::vec(0..100i32, 0..100)) {
        let sorted = bubble_sort(list);
        prop_assert!(is_sorted(&sorted));
    }

    #[test]
    fn test_sort_preserves_length(ref list in prop::collection::vec(0..100i32, 0..100)) {
        let sorted = bubble_sort(list);
        prop_assert_eq!(list.len(), sorted.len());
    }

    #[test]
    fn test_sort_preserves_elements(ref list in prop::collection::vec(0..100i32, 0..100)) {
        let mut original = list.clone();
        let mut sorted = bubble_sort(list);

        original.sort();
        prop_assert_eq!(original, sorted);
    }
}
*/
