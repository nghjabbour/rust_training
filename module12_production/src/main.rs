// Module 12: Production Rust
// This module explores best practices for using Rust in production:
// - Performance optimization techniques
// - Memory optimization patterns
// - Error handling at scale
// - Logging and monitoring
// - Configuration management
// - Database interaction patterns
// - Web service patterns
// - Deployment considerations

mod problems;

use std::sync::Arc;
use std::time::Instant;

fn main() {
    // Run the practice problems
    problems::run_exercises();

    // ===============================
    // 1. Performance Optimization Techniques
    // ===============================
    println!("\n1. Performance Optimization Techniques:");
    println!("----------------------------------");
    // Techniques for optimizing Rust code performance

    println!("Compile-time optimizations:");
    println!("  - Use release builds: cargo build --release");
    println!("  - Enable link-time optimization (LTO): lto = true");
    println!("  - Set codegen-units: codegen-units = 1");
    println!("  - Profile-guided optimization (PGO)");

    println!("\nAlgorithmic optimizations:");
    println!("  - Choose appropriate algorithms and data structures");
    println!("  - Minimize allocations and copies");
    println!("  - Use iterators and avoid unnecessary collections");
    println!("  - Consider custom data structures for specific use cases");

    println!("\nParallelism and concurrency:");
    println!("  - Use rayon for data parallelism");
    println!("  - Use tokio for async I/O");
    println!("  - Consider thread pools for CPU-bound tasks");
    println!("  - Use channels for communication between threads");

    println!("\nSIMD and low-level optimizations:");
    println!("  - Use SIMD intrinsics for numeric code");
    println!("  - Consider unsafe code for performance-critical sections");
    println!("  - Use packed data structures");
    println!("  - Align data for cache efficiency");

    // Example: Measuring performance
    let start = Instant::now();
    // Simulate some work
    let mut sum = 0;
    for i in 0..1_000_000 {
        sum += i;
    }
    let duration = start.elapsed();
    println!("\nExample timing: {:?}", duration);

    // ===============================
    // 2. Memory Optimization Patterns
    // ===============================
    println!("\n2. Memory Optimization Patterns:");
    println!("----------------------------");
    // Patterns for optimizing memory usage in Rust

    println!("Stack vs heap allocation:");
    println!("  - Prefer stack allocation for small, fixed-size data");
    println!("  - Use Box<T> for heap allocation");
    println!("  - Consider Rc<T> or Arc<T> for shared ownership");
    println!("  - Use references for borrowed access");

    println!("\nMinimizing allocations:");
    println!("  - Reuse allocations when possible");
    println!("  - Pre-allocate with capacity");
    println!("  - Use with_capacity for Vec and HashMap");
    println!("  - Consider custom allocators for specific use cases");

    println!("\nMemory layout optimizations:");
    println!("  - Order struct fields to minimize padding");
    println!("  - Use appropriate sized types");
    println!("  - Consider packed structs for space efficiency");
    println!("  - Use enums with discriminant optimization");

    println!("\nMemory pooling and arenas:");
    println!("  - Use object pools for frequently allocated objects");
    println!("  - Consider arena allocators for short-lived allocations");
    println!("  - Use typed_arena or bumpalo crates");

    // Example: Memory-efficient data structure
    struct CompactUser {
        // Order fields from largest to smallest to minimize padding
        name: String,     // 24 bytes
        email: String,    // 24 bytes
        login_count: u32, // 4 bytes
        is_active: bool,  // 1 byte
                          // 3 bytes padding to align to 8-byte boundary
    }

    // ===============================
    // 3. Error Handling at Scale
    // ===============================
    println!("\n3. Error Handling at Scale:");
    println!("------------------------");
    // Error handling patterns for large-scale applications

    println!("Custom error types:");
    println!("  - Define domain-specific error types");
    println!("  - Implement std::error::Error trait");
    println!("  - Provide context and source information");
    println!("  - Consider thiserror or anyhow crates");

    println!("\nError context and propagation:");
    println!("  - Use ? operator for error propagation");
    println!("  - Add context to errors with .context() or .with_context()");
    println!("  - Consider error source chains");
    println!("  - Use map_err for error transformation");

    println!("\nError handling strategies:");
    println!("  - Fail fast for unrecoverable errors");
    println!("  - Retry with backoff for transient errors");
    println!("  - Provide fallbacks for non-critical features");
    println!("  - Circuit breakers for external dependencies");

    println!("\nError reporting and monitoring:");
    println!("  - Structured logging for errors");
    println!("  - Error aggregation and analysis");
    println!("  - Alerting for critical errors");
    println!("  - Error rate monitoring");

    // Example: Custom error type
    #[derive(Debug)]
    enum AppError {
        Database(String),
        Network(String),
        Validation(String),
        NotFound(String),
    }

    impl std::fmt::Display for AppError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                AppError::Database(msg) => write!(f, "Database error: {}", msg),
                AppError::Network(msg) => write!(f, "Network error: {}", msg),
                AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
                AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            }
        }
    }

    impl std::error::Error for AppError {}

    // ===============================
    // 4. Logging and Monitoring
    // ===============================
    println!("\n4. Logging and Monitoring:");
    println!("----------------------");
    // Logging and monitoring patterns for production applications

    println!("Logging frameworks:");
    println!("  - log crate for logging facade");
    println!("  - env_logger for development");
    println!("  - slog for structured logging");
    println!("  - tracing for hierarchical spans");

    println!("\nLog levels and filtering:");
    println!("  - error: Error conditions");
    println!("  - warn: Warning conditions");
    println!("  - info: Informational messages");
    println!("  - debug: Debug-level messages");
    println!("  - trace: Trace-level messages");

    println!("\nStructured logging:");
    println!("  - Include context with log events");
    println!("  - Use key-value pairs");
    println!("  - Consider JSON format for machine processing");
    println!("  - Include request IDs for correlation");

    println!("\nMetrics and monitoring:");
    println!("  - Prometheus for metrics collection");
    println!("  - Grafana for visualization");
    println!("  - OpenTelemetry for distributed tracing");
    println!("  - Health checks and readiness probes");

    // Example: Structured logging
    println!("\nExample structured logging:");
    println!("  info!(user_id = 123, action = \"login\", \"User logged in\");");

    // ===============================
    // 5. Configuration Management
    // ===============================
    println!("\n5. Configuration Management:");
    println!("--------------------------");
    // Configuration management patterns for production applications

    println!("Configuration sources:");
    println!("  - Environment variables");
    println!("  - Configuration files (TOML, YAML, JSON)");
    println!("  - Command-line arguments");
    println!("  - Remote configuration services");

    println!("\nConfiguration libraries:");
    println!("  - config crate for multiple sources");
    println!("  - dotenv for .env files");
    println!("  - clap for command-line arguments");
    println!("  - serde for deserialization");

    println!("\nConfiguration patterns:");
    println!("  - Validate configuration at startup");
    println!("  - Provide sensible defaults");
    println!("  - Support overrides and layering");
    println!("  - Consider hot reloading for some settings");

    println!("\nSecrets management:");
    println!("  - Separate secrets from configuration");
    println!("  - Use environment variables for secrets");
    println!("  - Consider vault or AWS Secrets Manager");
    println!("  - Encrypt sensitive configuration");

    // Example: Configuration struct
    #[derive(Debug)]
    struct Config {
        server: ServerConfig,
        database: DatabaseConfig,
        logging: LoggingConfig,
    }

    #[derive(Debug)]
    struct ServerConfig {
        host: String,
        port: u16,
        workers: usize,
    }

    #[derive(Debug)]
    struct DatabaseConfig {
        url: String,
        max_connections: usize,
    }

    #[derive(Debug)]
    struct LoggingConfig {
        level: String,
        file: Option<String>,
    }

    // ===============================
    // 6. Database Interaction Patterns
    // ===============================
    println!("\n6. Database Interaction Patterns:");
    println!("------------------------------");
    // Patterns for interacting with databases in production

    println!("Database libraries:");
    println!("  - sqlx for SQL databases");
    println!("  - diesel for ORM");
    println!("  - mongodb for MongoDB");
    println!("  - redis for Redis");

    println!("\nConnection pooling:");
    println!("  - r2d2 for connection pooling");
    println!("  - deadpool for async connection pooling");
    println!("  - Configure pool size based on workload");
    println!("  - Monitor connection usage");

    println!("\nQuery patterns:");
    println!("  - Prepared statements for security and performance");
    println!("  - Transactions for atomicity");
    println!("  - Pagination for large result sets");
    println!("  - Consider query builders");

    println!("\nMigration and schema management:");
    println!("  - refinery for migrations");
    println!("  - sqlx-cli for migrations");
    println!("  - Version control for migrations");
    println!("  - Automated testing for migrations");

    // Example: Database connection pool
    println!("\nExample connection pool setup:");
    println!("  let pool = Pool::builder()");
    println!("      .max_size(15)");
    println!("      .build(manager)");
    println!("      .expect(\"Failed to create pool\");");

    // ===============================
    // 7. Web Service Patterns
    // ===============================
    println!("\n7. Web Service Patterns:");
    println!("---------------------");
    // Patterns for building web services in production

    println!("Web frameworks:");
    println!("  - actix-web for high performance");
    println!("  - rocket for developer experience");
    println!("  - axum for tokio integration");
    println!("  - warp for composable filters");

    println!("\nAPI design patterns:");
    println!("  - RESTful APIs with proper status codes");
    println!("  - GraphQL for flexible queries");
    println!("  - gRPC for service-to-service communication");
    println!("  - WebSockets for real-time communication");

    println!("\nMiddleware and filters:");
    println!("  - Authentication and authorization");
    println!("  - Request logging and tracing");
    println!("  - Rate limiting and throttling");
    println!("  - CORS and security headers");

    println!("\nSerialization and validation:");
    println!("  - serde for serialization/deserialization");
    println!("  - validator for input validation");
    println!("  - JSON Schema for API contracts");
    println!("  - OpenAPI for documentation");

    // Example: Web service with middleware
    println!("\nExample web service setup:");
    println!("  let app = App::new()");
    println!("      .wrap(Logger::default())");
    println!("      .wrap(Cors::default())");
    println!("      .service(web::resource(\"/users\")");
    println!("          .route(web::get().to(get_users))");
    println!("          .route(web::post().to(create_user)))");

    // ===============================
    // 8. Deployment Considerations
    // ===============================
    println!("\n8. Deployment Considerations:");
    println!("---------------------------");
    // Considerations for deploying Rust applications in production

    println!("Build and packaging:");
    println!("  - Static linking for portability");
    println!("  - musl target for smaller binaries");
    println!("  - Docker multi-stage builds");
    println!("  - Consider cross compilation");

    println!("\nContainer orchestration:");
    println!("  - Kubernetes for container orchestration");
    println!("  - Health checks and readiness probes");
    println!("  - Resource limits and requests");
    println!("  - Horizontal pod autoscaling");

    println!("\nObservability:");
    println!("  - Distributed tracing with OpenTelemetry");
    println!("  - Metrics collection with Prometheus");
    println!("  - Log aggregation with ELK or Loki");
    println!("  - Alerting and dashboards");

    println!("\nContinuous deployment:");
    println!("  - CI/CD pipelines");
    println!("  - Automated testing");
    println!("  - Canary deployments");
    println!("  - Blue-green deployments");

    // Example: Dockerfile for Rust application
    println!("\nExample Dockerfile:");
    println!("  FROM rust:1.68 as builder");
    println!("  WORKDIR /app");
    println!("  COPY . .");
    println!("  RUN cargo build --release");
    println!("");
    println!("  FROM debian:bullseye-slim");
    println!("  COPY --from=builder /app/target/release/my_app /usr/local/bin/");
    println!("  CMD [\"my_app\"]");
}

// Notes on Production Rust:
//
// 1. Performance Considerations
//    - Rust provides zero-cost abstractions
//    - Release builds with optimizations
//    - Profile-guided optimization
//    - SIMD and low-level optimizations
//    - Parallelism with rayon and tokio
//
// 2. Memory Management
//    - Stack vs heap allocation
//    - Minimizing allocations
//    - Memory layout optimizations
//    - Custom allocators for specific use cases
//    - Memory pooling and arenas
//
// 3. Error Handling
//    - Custom error types
//    - Error context and propagation
//    - Error handling strategies
//    - Error reporting and monitoring
//    - Circuit breakers and fallbacks
//
// 4. Observability
//    - Structured logging
//    - Metrics collection
//    - Distributed tracing
//    - Health checks and readiness probes
//    - Alerting and dashboards
//
// 5. Deployment
//    - Static linking for portability
//    - Container orchestration
//    - CI/CD pipelines
//    - Canary and blue-green deployments
//    - Resource management and scaling

// Try experimenting with these concepts:
// 1. Profile and optimize a Rust application
// 2. Implement a custom error type with context
// 3. Set up structured logging and metrics
// 4. Create a containerized Rust application
// 5. Implement a connection pool for database access
