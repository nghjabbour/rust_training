// Module 9: Practice Problems
// These exercises focus on advanced pattern matching in Rust.
// Each problem includes detailed explanations of the underlying concepts
// and how pattern matching enables expressive and safe code.

pub fn run_exercises() {
    println!("Module 9 Exercises - Advanced Pattern Matching");
    println!("=========================================\n");

    exercise1();
    exercise2();
    exercise3();
    exercise4();
    exercise5();
}

// Exercise 1: Pattern Matching with Complex Enums
// This exercise demonstrates:
// - Destructuring nested enums
// - Match guards with complex conditions
// - Binding parts of patterns
fn exercise1() {
    println!("Exercise 1: Pattern Matching with Complex Enums");
    println!("----------------------------------------");
    println!("TODO: Implement the process_event function\n");

    // Define a complex event system
    #[derive(Debug)]
    enum UserId {
        Anonymous,
        Registered(u64),
        Admin(String),
    }

    #[derive(Debug)]
    enum Resource {
        File { name: String, size: usize },
        Database { table: String, id: u64 },
        Network { url: String, port: u16 },
    }

    #[derive(Debug)]
    enum Action {
        Read,
        Write(String),
        Delete,
        Create { template: Option<String> },
    }

    #[derive(Debug)]
    struct Event {
        user: UserId,
        resource: Resource,
        action: Action,
        timestamp: u64,
    }

    // Process an event and return a message describing it
    fn process_event(event: &Event) -> String {
        unimplemented!("Implement process_event");
    }

    // Test your implementation:
    // let events = vec![
    //     Event {
    //         user: UserId::Anonymous,
    //         resource: Resource::File { name: "public.txt".to_string(), size: 100 },
    //         action: Action::Read,
    //         timestamp: 1621022800,
    //     },
    //     Event {
    //         user: UserId::Registered(42),
    //         resource: Resource::Database { table: "users".to_string(), id: 123 },
    //         action: Action::Write("update name".to_string()),
    //         timestamp: 1621022900,
    //     },
    //     Event {
    //         user: UserId::Admin("root".to_string()),
    //         resource: Resource::Network { url: "api.example.com".to_string(), port: 443 },
    //         action: Action::Create { template: Some("default".to_string()) },
    //         timestamp: 1621023000,
    //     },
    // ];
    //
    // for event in &events {
    //     println!("{}", process_event(event));
    // }
}

// Exercise 2: Custom DSL Parser
// This exercise demonstrates:
// - Using pattern matching for parsing
// - Building an AST from tokens
// - Implementing a simple interpreter
fn exercise2() {
    println!("\nExercise 2: Custom DSL Parser");
    println!("------------------------");
    println!("TODO: Implement the parse and evaluate functions\n");

    // A simple expression language
    #[derive(Debug, PartialEq)]
    enum Token {
        Number(i32),
        Plus,
        Minus,
        Multiply,
        Divide,
        LeftParen,
        RightParen,
    }

    #[derive(Debug, PartialEq)]
    enum Expr {
        Number(i32),
        BinaryOp {
            op: Op,
            left: Box<Expr>,
            right: Box<Expr>,
        },
    }

    #[derive(Debug, PartialEq)]
    enum Op {
        Add,
        Subtract,
        Multiply,
        Divide,
    }

    // Tokenize a string into tokens
    fn tokenize(input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                '0'..='9' => {
                    let mut number = 0;
                    while let Some(&c) = chars.peek() {
                        if c.is_digit(10) {
                            number = number * 10 + c.to_digit(10).unwrap() as i32;
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(number));
                }
                '+' => {
                    tokens.push(Token::Plus);
                    chars.next();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    chars.next();
                }
                '*' => {
                    tokens.push(Token::Multiply);
                    chars.next();
                }
                '/' => {
                    tokens.push(Token::Divide);
                    chars.next();
                }
                '(' => {
                    tokens.push(Token::LeftParen);
                    chars.next();
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    chars.next();
                }
                ' ' => {
                    chars.next();
                }
                _ => panic!("Unexpected character: {}", c),
            }
        }

        tokens
    }

    // Parse tokens into an expression tree
    fn parse(tokens: &[Token]) -> Result<Expr, String> {
        unimplemented!("Implement parse");
    }

    // Evaluate an expression tree
    fn evaluate(expr: &Expr) -> i32 {
        unimplemented!("Implement evaluate");
    }

    // Test your implementation:
    // let input = "3 + 4 * (2 - 1)";
    // let tokens = tokenize(input);
    // println!("Tokens: {:?}", tokens);
    //
    // match parse(&tokens) {
    //     Ok(expr) => {
    //         println!("Expression: {:?}", expr);
    //         println!("Result: {}", evaluate(&expr));
    //     }
    //     Err(e) => println!("Parse error: {}", e),
    // }
}

// Exercise 3: State Machine with Pattern Matching
// This exercise demonstrates:
// - Using enums to represent states
// - Pattern matching for state transitions
// - Complex state management
fn exercise3() {
    println!("\nExercise 3: State Machine with Pattern Matching");
    println!("------------------------------------------");
    println!("TODO: Implement the TrafficLight struct and its methods\n");

    // A traffic light state machine
    #[derive(Debug, PartialEq)]
    enum LightState {
        Red,
        Yellow,
        Green,
        BlinkingYellow,
        Off,
    }

    #[derive(Debug, PartialEq)]
    enum Event {
        TimerTick,
        PowerOutage,
        PowerRestored,
        CarDetected,
        EmergencyVehicle,
    }

    struct TrafficLight {
        state: LightState,
        timer: u32,
    }

    impl TrafficLight {
        // Create a new traffic light
        fn new() -> Self {
            unimplemented!("Implement TrafficLight::new");
        }

        // Process an event and update the state
        fn process_event(&mut self, event: Event) {
            unimplemented!("Implement process_event");
        }

        // Get the current state
        fn state(&self) -> &LightState {
            unimplemented!("Implement state");
        }

        // Get the current timer value
        fn timer(&self) -> u32 {
            unimplemented!("Implement timer");
        }
    }

    // Test your implementation:
    // let mut light = TrafficLight::new();
    // println!("Initial state: {:?}, Timer: {}", light.state(), light.timer());
    //
    // let events = vec![
    //     Event::TimerTick,
    //     Event::TimerTick,
    //     Event::PowerOutage,
    //     Event::PowerRestored,
    //     Event::EmergencyVehicle,
    //     Event::TimerTick,
    // ];
    //
    // for event in events {
    //     println!("Processing event: {:?}", event);
    //     light.process_event(event);
    //     println!("New state: {:?}, Timer: {}", light.state(), light.timer());
    // }
}

// Exercise 4: Advanced Error Handling
// This exercise demonstrates:
// - Pattern matching for error handling
// - Custom error types
// - Error context and propagation
fn exercise4() {
    println!("\nExercise 4: Advanced Error Handling");
    println!("------------------------------");
    println!("TODO: Implement the parse_config function\n");

    // Custom error type
    #[derive(Debug)]
    enum ConfigError {
        IoError(std::io::Error),
        ParseError { line: usize, message: String },
        ValidationError(String),
        MissingField(String),
    }

    impl std::fmt::Display for ConfigError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                ConfigError::IoError(e) => write!(f, "IO error: {}", e),
                ConfigError::ParseError { line, message } => {
                    write!(f, "Parse error at line {}: {}", line, message)
                }
                ConfigError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
                ConfigError::MissingField(field) => write!(f, "Missing field: {}", field),
            }
        }
    }

    impl std::error::Error for ConfigError {}

    impl From<std::io::Error> for ConfigError {
        fn from(error: std::io::Error) -> Self {
            ConfigError::IoError(error)
        }
    }

    // Configuration type
    #[derive(Debug)]
    struct Config {
        server: String,
        port: u16,
        timeout: u32,
        max_connections: Option<u32>,
    }

    // Parse a configuration string
    fn parse_config(input: &str) -> Result<Config, ConfigError> {
        unimplemented!("Implement parse_config");
    }

    // Test your implementation:
    // let config_str = r#"
    //     server=localhost
    //     port=8080
    //     timeout=30
    //     # max_connections is optional
    //     # max_connections=100
    // "#;
    //
    // match parse_config(config_str) {
    //     Ok(config) => println!("Parsed config: {:?}", config),
    //     Err(e) => println!("Error: {}", e),
    // }
    //
    // // Test with invalid config
    // let invalid_config = r#"
    //     server=localhost
    //     port=invalid
    //     timeout=30
    // "#;
    //
    // match parse_config(invalid_config) {
    //     Ok(config) => println!("Parsed config: {:?}", config),
    //     Err(e) => println!("Error: {}", e),
    // }
}

// Exercise 5: Pattern Matching Optimization
// This exercise demonstrates:
// - Optimizing pattern matching
// - Benchmarking different approaches
// - Understanding compiler optimizations
fn exercise5() {
    println!("\nExercise 5: Pattern Matching Optimization");
    println!("---------------------------------");
    println!("TODO: Implement the optimized_match function\n");

    // A large enum with many variants
    #[derive(Debug, Clone, Copy)]
    enum Command {
        Noop,
        Clear,
        Reset,
        Move(i32, i32),
        Color(u8, u8, u8),
        Line(i32, i32, i32, i32),
        Rect(i32, i32, i32, i32),
        Circle(i32, i32, i32),
        Text(char, i32, i32),
        // Many more variants...
    }

    // Process a command using naive pattern matching
    fn naive_match(cmd: Command) -> String {
        match cmd {
            Command::Noop => "No operation".to_string(),
            Command::Clear => "Clear screen".to_string(),
            Command::Reset => "Reset state".to_string(),
            Command::Move(x, y) => format!("Move to ({}, {})", x, y),
            Command::Color(r, g, b) => format!("Set color to RGB({}, {}, {})", r, g, b),
            Command::Line(x1, y1, x2, y2) => {
                format!("Draw line from ({}, {}) to ({}, {})", x1, y1, x2, y2)
            }
            Command::Rect(x, y, w, h) => {
                format!("Draw rectangle at ({}, {}) with size {}x{}", x, y, w, h)
            }
            Command::Circle(x, y, r) => format!("Draw circle at ({}, {}) with radius {}", x, y, r),
            Command::Text(c, x, y) => format!("Draw text '{}' at ({}, {})", c, x, y),
        }
    }

    // Process a command using optimized pattern matching
    // Hint: Consider grouping similar variants, using guards, etc.
    fn optimized_match(cmd: Command) -> String {
        unimplemented!("Implement optimized_match");
    }

    // Test your implementation:
    // let commands = vec![
    //     Command::Noop,
    //     Command::Move(10, 20),
    //     Command::Color(255, 0, 0),
    //     Command::Line(0, 0, 100, 100),
    //     Command::Rect(10, 10, 50, 50),
    //     Command::Circle(50, 50, 25),
    //     Command::Text('A', 10, 10),
    // ];
    //
    // // Compare results
    // for cmd in &commands {
    //     let naive = naive_match(*cmd);
    //     let optimized = optimized_match(*cmd);
    //     println!("Command: {:?}", cmd);
    //     println!("  Naive: {}", naive);
    //     println!("  Optimized: {}", optimized);
    //     assert_eq!(naive, optimized);
    // }
    //
    // // In a real benchmark, you would use criterion or similar
    // // to measure performance differences
}

/* Example Solutions (Try to solve the exercises before looking at these!)

// Exercise 1 Solution:
fn process_event(event: &Event) -> String {
    match event {
        // Anonymous users
        Event { user: UserId::Anonymous, resource: Resource::File { name, .. }, action: Action::Read, .. } => {
            format!("Anonymous user read file '{}'", name)
        }

        // Registered users
        Event { user: UserId::Registered(id), resource: Resource::File { name, size }, action: Action::Read, .. } => {
            format!("User {} read file '{}' ({} bytes)", id, name, size)
        }

        Event { user: UserId::Registered(id), resource: Resource::Database { table, id: record_id }, action: Action::Write(data), .. } => {
            format!("User {} wrote '{}' to table '{}', record {}", id, data, table, record_id)
        }

        // Admin users
        Event { user: UserId::Admin(name), action: Action::Delete, resource, .. } => {
            match resource {
                Resource::File { name: file_name, .. } => {
                    format!("Admin '{}' deleted file '{}'", name, file_name)
                }
                Resource::Database { table, id } => {
                    format!("Admin '{}' deleted record {} from table '{}'", name, id, table)
                }
                _ => format!("Admin '{}' deleted a resource", name)
            }
        }

        // Create actions with templates
        Event { user, resource, action: Action::Create { template: Some(template) }, .. } => {
            let user_str = match user {
                UserId::Anonymous => "Anonymous user".to_string(),
                UserId::Registered(id) => format!("User {}", id),
                UserId::Admin(name) => format!("Admin '{}'", name),
            };

            let resource_str = match resource {
                Resource::File { name, .. } => format!("file '{}'", name),
                Resource::Database { table, .. } => format!("database table '{}'", table),
                Resource::Network { url, .. } => format!("network resource '{}'", url),
            };

            format!("{} created {} using template '{}'", user_str, resource_str, template)
        }

        // Fallback for other cases
        _ => format!("Unhandled event: {:?}", event),
    }
}

// Exercise 2 Solution:
fn parse(tokens: &[Token]) -> Result<Expr, String> {
    // Simple recursive descent parser
    fn parse_expr(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
        let left = parse_term(tokens, pos)?;

        if *pos < tokens.len() {
            match tokens[*pos] {
                Token::Plus => {
                    *pos += 1;
                    let right = parse_expr(tokens, pos)?;
                    Ok(Expr::BinaryOp {
                        op: Op::Add,
                        left: Box::new(left),
                        right: Box::new(right),
                    })
                }
                Token::Minus => {
                    *pos += 1;
                    let right = parse_expr(tokens, pos)?;
                    Ok(Expr::BinaryOp {
                        op: Op::Subtract,
                        left: Box::new(left),
                        right: Box::new(right),
                    })
                }
                _ => Ok(left),
            }
        } else {
            Ok(left)
        }
    }

    fn parse_term(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
        let left = parse_factor(tokens, pos)?;

        if *pos < tokens.len() {
            match tokens[*pos] {
                Token::Multiply => {
                    *pos += 1;
                    let right = parse_term(tokens, pos)?;
                    Ok(Expr::BinaryOp {
                        op: Op::Multiply,
                        left: Box::new(left),
                        right: Box::new(right),
                    })
                }
                Token::Divide => {
                    *pos += 1;
                    let right = parse_term(tokens, pos)?;
                    Ok(Expr::BinaryOp {
                        op: Op::Divide,
                        left: Box::new(left),
                        right: Box::new(right),
                    })
                }
                _ => Ok(left),
            }
        } else {
            Ok(left)
        }
    }

    fn parse_factor(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
        if *pos >= tokens.len() {
            return Err("Unexpected end of input".to_string());
        }

        match tokens[*pos] {
            Token::Number(n) => {
                *pos += 1;
                Ok(Expr::Number(n))
            }
            Token::LeftParen => {
                *pos += 1;
                let expr = parse_expr(tokens, pos)?;

                if *pos >= tokens.len() || tokens[*pos] != Token::RightParen {
                    return Err("Expected closing parenthesis".to_string());
                }

                *pos += 1;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token: {:?}", tokens[*pos])),
        }
    }

    let mut pos = 0;
    let expr = parse_expr(tokens, &mut pos)?;

    if pos < tokens.len() {
        Err(format!("Unexpected tokens after expression: {:?}", &tokens[pos..]))
    } else {
        Ok(expr)
    }
}

fn evaluate(expr: &Expr) -> i32 {
    match expr {
        Expr::Number(n) => *n,
        Expr::BinaryOp { op, left, right } => {
            let left_val = evaluate(left);
            let right_val = evaluate(right);

            match op {
                Op::Add => left_val + right_val,
                Op::Subtract => left_val - right_val,
                Op::Multiply => left_val * right_val,
                Op::Divide => left_val / right_val,
            }
        }
    }
}

// Exercise 3 Solution:
impl TrafficLight {
    fn new() -> Self {
        TrafficLight {
            state: LightState::Red,
            timer: 0,
        }
    }

    fn process_event(&mut self, event: Event) {
        match (event, &self.state) {
            // Power outage transitions to Off state
            (Event::PowerOutage, _) => {
                self.state = LightState::Off;
                self.timer = 0;
            }

            // Power restored transitions to Red state
            (Event::PowerRestored, LightState::Off) => {
                self.state = LightState::Red;
                self.timer = 0;
            }

            // Emergency vehicle transitions to BlinkingYellow
            (Event::EmergencyVehicle, _) if self.state != LightState::Off => {
                self.state = LightState::BlinkingYellow;
                self.timer = 0;
            }

            // Normal traffic light cycle
            (Event::TimerTick, LightState::Red) => {
                self.timer += 1;
                if self.timer >= 30 {
                    self.state = LightState::Green;
                    self.timer = 0;
                }
            }

            (Event::TimerTick, LightState::Green) => {
                self.timer += 1;
                if self.timer >= 20 {
                    self.state = LightState::Yellow;
                    self.timer = 0;
                }
            }

            (Event::TimerTick, LightState::Yellow) => {
                self.timer += 1;
                if self.timer >= 5 {
                    self.state = LightState::Red;
                    self.timer = 0;
                }
            }

            (Event::TimerTick, LightState::BlinkingYellow) => {
                self.timer += 1;
                if self.timer >= 60 {
                    self.state = LightState::Red;
                    self.timer = 0;
                }
            }

            // Car detection can shorten green light but not extend it
            (Event::CarDetected, LightState::Green) => {
                if self.timer < 10 {
                    self.timer = 10;
                }
            }

            // Default case: do nothing
            _ => {}
        }
    }

    fn state(&self) -> &LightState {
        &self.state
    }

    fn timer(&self) -> u32 {
        self.timer
    }
}

// Exercise 4 Solution:
fn parse_config(input: &str) -> Result<Config, ConfigError> {
    let mut server = None;
    let mut port = None;
    let mut timeout = None;
    let mut max_connections = None;

    for (line_num, line) in input.lines().enumerate() {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse key-value pairs
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 {
            return Err(ConfigError::ParseError {
                line: line_num + 1,
                message: "Invalid line format".to_string(),
            });
        }

        let key = parts[0].trim();
        let value = parts[1].trim();

        match key {
            "server" => server = Some(value.to_string()),
            "port" => {
                port = Some(value.parse::<u16>().map_err(|_| ConfigError::ParseError {
                    line: line_num + 1,
                    message: "Invalid port number".to_string(),
                })?);
            }
            "timeout" => {
                timeout = Some(value.parse::<u32>().map_err(|_| ConfigError::ParseError {
                    line: line_num + 1,
                    message: "Invalid timeout value".to_string(),
                })?);
            }
            "max_connections" => {
                max_connections = Some(value.parse::<u32>().map_err(|_| ConfigError::ParseError {
                    line: line_num + 1,
                    message: "Invalid max_connections value".to_string(),
                })?);
            }
            _ => {
                return Err(ConfigError::ParseError {
                    line: line_num + 1,
                    message: format!("Unknown configuration key: {}", key),
                });
            }
        }
    }

    // Validate required fields
    let server = server.ok_or_else(|| ConfigError::MissingField("server".to_string()))?;
    let port = port.ok_or_else(|| ConfigError::MissingField("port".to_string()))?;
    let timeout = timeout.ok_or_else(|| ConfigError::MissingField("timeout".to_string()))?;

    // Validate values
    if port == 0 {
        return Err(ConfigError::ValidationError("Port cannot be 0".to_string()));
    }

    if timeout == 0 {
        return Err(ConfigError::ValidationError("Timeout cannot be 0".to_string()));
    }

    Ok(Config {
        server,
        port,
        timeout,
        max_connections,
    })
}

// Exercise 5 Solution:
fn optimized_match(cmd: Command) -> String {
    // Group commands by their "type" to reduce the number of comparisons
    match cmd {
        // Simple commands with no parameters
        Command::Noop | Command::Clear | Command::Reset => {
            match cmd {
                Command::Noop => "No operation".to_string(),
                Command::Clear => "Clear screen".to_string(),
                Command::Reset => "Reset state".to_string(),
                _ => unreachable!(),
            }
        }

        // Commands with position parameters
        Command::Move(x, y) => format!("Move to ({}, {})", x, y),
        Command::Text(c, x, y) => format!("Draw text '{}' at ({}, {})", c, x, y),

        // Drawing commands with multiple parameters
        cmd @ Command::Line(_, _, _, _) | cmd @ Command::Rect(_, _, _, _) => {
            match cmd {
                Command::Line(x1, y1, x2, y2) => {
                    format!("Draw line from ({}, {}) to ({}, {})", x1, y1, x2, y2)
                }
                Command::Rect(x, y, w, h) => {
                    format!("Draw rectangle at ({}, {}) with size {}x{}", x, y, w, h)
                }
                _ => unreachable!(),
            }
        }

        // Other drawing commands
        Command::Circle(x, y, r) => format!("Draw circle at ({}, {}) with radius {}", x, y, r),

        // Color command
        Command::Color(r, g, b) => format!("Set color to RGB({}, {}, {})", r, g, b),
    }
}

// Note: In a real optimization scenario, you would benchmark different approaches
// and choose the one that performs best for your specific use case.
*/
