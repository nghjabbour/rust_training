// Module 3: Practice Problems
// These exercises focus on Rust's type system, memory layout, and pattern matching.
// Each problem includes detailed explanations of the underlying concepts and
// comparisons with how similar problems would be solved in other languages.

use std::collections::HashMap;
use std::mem;

pub fn run_exercises() {
    println!("Module 3 Exercises - Type System Deep Dive");
    println!("=====================================\n");

    exercise1();
    exercise2();
    exercise3();
    exercise4();
    exercise5();
}

// Exercise 1: Memory Layout Optimization
// This exercise demonstrates:
// - How struct field ordering affects memory usage
// - Alignment and padding in structs
// - Performance implications of memory layout
fn exercise1() {
    println!("Exercise 1: Memory Layout Optimization");
    println!("----------------------------------");
    println!("TODO: Implement optimized versions of the structs\n");

    // Original struct with suboptimal memory layout
    #[derive(Debug)]
    struct PersonBad {
        active: bool,  // 1 byte + 7 bytes padding
        name: String,  // 24 bytes
        age: u32,      // 4 bytes + 4 bytes padding
        email: String, // 24 bytes
    }

    // TODO: Create an optimized version of the struct
    // Hint: Reorder fields to minimize padding
    #[derive(Debug)]
    struct PersonGood {
        name: String,  // 24 bytes
        email: String, // 24 bytes
        age: u32,      // 4 bytes
        active: bool,  // 1 byte + 3 bytes padding
    }

    // Function to analyze and compare memory layouts
    fn analyze_layouts() {
        unimplemented!("Implement analyze_layouts to compare struct sizes");
    }

    // Test your implementation:
    // analyze_layouts();
}

// Exercise 2: Tagged Union Implementation
// This exercise demonstrates:
// - How Rust enums work under the hood
// - Memory layout of different enum variants
// - Pattern matching optimization
fn exercise2() {
    println!("\nExercise 2: Tagged Union Implementation");
    println!("-----------------------------------");
    println!("TODO: Implement the JsonValue enum and its methods\n");

    // A simplified JSON value type
    #[derive(Debug)]
    enum JsonValue {
        Null,
        Boolean(bool),
        Number(f64),
        String(String),
        Array(Vec<JsonValue>),
        Object(HashMap<String, JsonValue>),
    }

    impl JsonValue {
        // Create a new JSON value from a Rust value
        fn from_bool(b: bool) -> Self {
            unimplemented!("Implement from_bool");
        }

        // Safely get a boolean value
        fn as_bool(&self) -> Option<bool> {
            unimplemented!("Implement as_bool");
        }

        // Pretty print the JSON value
        fn pretty_print(&self, indent: usize) -> String {
            unimplemented!("Implement pretty_print");
        }
    }

    // Test your implementation:
    // let json = JsonValue::Object({
    //     let mut map = HashMap::new();
    //     map.insert("name".to_string(), JsonValue::String("Alice".to_string()));
    //     map.insert("age".to_string(), JsonValue::Number(30.0));
    //     map.insert("is_student".to_string(), JsonValue::from_bool(true));
    //     map
    // });
    // println!("JSON:\n{}", json.pretty_print(2));
}

// Exercise 3: Advanced Pattern Matching
// This exercise demonstrates:
// - Complex pattern matching scenarios
// - Match guards and bindings
// - Exhaustiveness checking
// - Pattern matching optimization
fn exercise3() {
    println!("\nExercise 3: Advanced Pattern Matching");
    println!("----------------------------------");
    println!("TODO: Implement the parse_log_entry function\n");

    // Log entry types
    #[derive(Debug)]
    enum LogLevel {
        Debug,
        Info,
        Warning,
        Error,
    }

    #[derive(Debug)]
    struct LogEntry {
        level: LogLevel,
        timestamp: u64,
        message: String,
        context: HashMap<String, String>,
    }

    // Parse a log entry from a string with format:
    // LEVEL [timestamp] message {key1=value1, key2=value2}
    fn parse_log_entry(line: &str) -> Option<LogEntry> {
        unimplemented!("Implement parse_log_entry");
    }

    // Test your implementation:
    // let log_lines = vec![
    //     "ERROR [1621234567] Failed to connect {host=db.example.com, port=5432}",
    //     "INFO [1621234568] User logged in {user_id=123, ip=192.168.1.1}",
    //     "DEBUG [1621234569] Cache miss {key=user_123, cache=users}",
    // ];
    //
    // for line in log_lines {
    //     match parse_log_entry(line) {
    //         Some(entry) => println!("Parsed: {:?}", entry),
    //         None => println!("Failed to parse: {}", line),
    //     }
    // }
}

// Exercise 4: Type State Programming
// This exercise demonstrates:
// - Using the type system to enforce state machine constraints
// - Zero-cost abstractions
// - Compile-time state checking
fn exercise4() {
    println!("\nExercise 4: Type State Programming");
    println!("--------------------------------");
    println!("TODO: Implement the Document state machine\n");

    // Document states
    struct Draft;
    struct UnderReview;
    struct Published;

    // Document with type-level state
    struct Document<State> {
        content: String,
        state: std::marker::PhantomData<State>,
    }

    // Implement state transitions
    impl Document<Draft> {
        fn new(content: String) -> Self {
            unimplemented!("Implement new");
        }

        fn submit_for_review(self) -> Document<UnderReview> {
            unimplemented!("Implement submit_for_review");
        }
    }

    impl Document<UnderReview> {
        fn approve(self) -> Document<Published> {
            unimplemented!("Implement approve");
        }

        fn reject(self) -> Document<Draft> {
            unimplemented!("Implement reject");
        }
    }

    impl<State> Document<State> {
        fn content(&self) -> &str {
            &self.content
        }
    }

    // Test your implementation:
    // let doc = Document::new("Draft content".to_string());
    // let doc = doc.submit_for_review();
    // let doc = doc.approve();
    // println!("Published content: {}", doc.content());
}

// Exercise 5: Error Type Design
// This exercise demonstrates:
// - Custom error type design
// - Error conversion and the From trait
// - Error handling best practices
// - Error context and wrapping
fn exercise5() {
    println!("\nExercise 5: Error Type Design");
    println!("---------------------------");
    println!("TODO: Implement the Configuration system\n");

    // Custom error type
    #[derive(Debug)]
    enum ConfigError {
        IoError(std::io::Error),
        ParseError { line: usize, message: String },
        ValidationError(String),
        MissingField(String),
    }

    // Configuration type
    struct Configuration {
        database_url: String,
        port: u16,
        api_key: Option<String>,
        features: Vec<String>,
    }

    impl Configuration {
        // Parse configuration from string
        fn from_string(content: &str) -> Result<Self, ConfigError> {
            unimplemented!("Implement from_string");
        }

        // Validate configuration
        fn validate(&self) -> Result<(), ConfigError> {
            unimplemented!("Implement validate");
        }
    }

    // Implement From for std::io::Error
    impl From<std::io::Error> for ConfigError {
        fn from(error: std::io::Error) -> Self {
            unimplemented!("Implement From for std::io::Error");
        }
    }

    // Test your implementation:
    // let config_str = r#"
    //     database_url=postgres://localhost/mydb
    //     port=8080
    //     features=["auth", "api", "web"]
    // "#;
    //
    // match Configuration::from_string(config_str) {
    //     Ok(config) => match config.validate() {
    //         Ok(()) => println!("Valid configuration"),
    //         Err(e) => println!("Invalid configuration: {:?}", e),
    //     },
    //     Err(e) => println!("Failed to parse configuration: {:?}", e),
    // }
}

/* Example Solutions (Try to solve the exercises before looking at these!)

// Exercise 1 Solution:
#[derive(Debug)]
struct PersonGood {
    name: String,     // 24 bytes
    email: String,    // 24 bytes
    age: u32,        // 4 bytes
    active: bool,    // 1 byte + 3 bytes padding
}

fn analyze_layouts() {
    println!("PersonBad size: {} bytes", mem::size_of::<PersonBad>());
    println!("PersonGood size: {} bytes", mem::size_of::<PersonGood>());
    println!("Memory saved: {} bytes",
        mem::size_of::<PersonBad>() - mem::size_of::<PersonGood>());
}

// Exercise 2 Solution:
impl JsonValue {
    fn from_bool(b: bool) -> Self {
        JsonValue::Boolean(b)
    }

    fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    fn pretty_print(&self, indent: usize) -> String {
        match self {
            JsonValue::Null => "null".to_string(),
            JsonValue::Boolean(b) => b.to_string(),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::String(s) => format!("\"{}\"", s),
            JsonValue::Array(arr) => {
                let items: Vec<String> = arr
                    .iter()
                    .map(|v| format!("{}{}",
                        " ".repeat(indent + 2),
                        v.pretty_print(indent + 2)))
                    .collect();
                format!("[\n{}\n{}]",
                    items.join(",\n"),
                    " ".repeat(indent))
            }
            JsonValue::Object(obj) => {
                let items: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("{}\"{}\": {}",
                        " ".repeat(indent + 2),
                        k,
                        v.pretty_print(indent + 2)))
                    .collect();
                format!("{{\n{}\n{}}}",
                    items.join(",\n"),
                    " ".repeat(indent))
            }
        }
    }
}

// Exercise 3 Solution:
fn parse_log_entry(line: &str) -> Option<LogEntry> {
    let parts: Vec<&str> = line.splitn(4, ' ').collect();
    if parts.len() != 4 {
        return None;
    }

    let level = match parts[0] {
        "DEBUG" => LogLevel::Debug,
        "INFO" => LogLevel::Info,
        "WARNING" => LogLevel::Warning,
        "ERROR" => LogLevel::Error,
        _ => return None,
    };

    let timestamp = parts[1]
        .trim_matches(|c| c == '[' || c == ']')
        .parse()
        .ok()?;

    let message = parts[2].to_string();

    let context = parts[3]
        .trim_matches(|c| c == '{' || c == '}')
        .split(',')
        .filter_map(|pair| {
            let kv: Vec<&str> = pair.split('=').collect();
            if kv.len() == 2 {
                Some((kv[0].trim().to_string(), kv[1].trim().to_string()))
            } else {
                None
            }
        })
        .collect();

    Some(LogEntry {
        level,
        timestamp,
        message,
        context,
    })
}

// Exercise 4 Solution:
impl Document<Draft> {
    fn new(content: String) -> Self {
        Document {
            content,
            state: std::marker::PhantomData,
        }
    }

    fn submit_for_review(self) -> Document<UnderReview> {
        Document {
            content: self.content,
            state: std::marker::PhantomData,
        }
    }
}

// Exercise 5 Solution:
impl Configuration {
    fn from_string(content: &str) -> Result<Self, ConfigError> {
        let mut config = Configuration {
            database_url: String::new(),
            port: 0,
            api_key: None,
            features: Vec::new(),
        };

        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() != 2 {
                return Err(ConfigError::ParseError {
                    line: line_num + 1,
                    message: "Invalid line format".to_string(),
                });
            }

            match parts[0].trim() {
                "database_url" => config.database_url = parts[1].trim().to_string(),
                "port" => config.port = parts[1].trim().parse().map_err(|_|
                    ConfigError::ParseError {
                        line: line_num + 1,
                        message: "Invalid port number".to_string(),
                    })?,
                "api_key" => config.api_key = Some(parts[1].trim().to_string()),
                "features" => {
                    let features = parts[1]
                        .trim_matches(|c| c == '[' || c == ']')
                        .split(',')
                        .map(|s| s.trim().trim_matches('"').to_string())
                        .collect();
                    config.features = features;
                }
                _ => return Err(ConfigError::ParseError {
                    line: line_num + 1,
                    message: format!("Unknown field: {}", parts[0]),
                }),
            }
        }

        Ok(config)
    }

    fn validate(&self) -> Result<(), ConfigError> {
        if self.database_url.is_empty() {
            return Err(ConfigError::MissingField("database_url".to_string()));
        }
        if self.port == 0 {
            return Err(ConfigError::ValidationError("Port cannot be 0".to_string()));
        }
        Ok(())
    }
}
*/
