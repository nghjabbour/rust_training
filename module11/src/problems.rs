// Module 11: Practice Problems
// These exercises focus on Rust tooling and ecosystem.
// Each problem includes detailed explanations of the underlying concepts
// and how to effectively use Rust's tools and ecosystem.

pub fn run_exercises() {
    println!("Module 11 Exercises - Rust Tooling and Ecosystem");
    println!("==========================================\n");

    exercise1();
    exercise2();
    exercise3();
    exercise4();
    exercise5();
}

// Exercise 1: Cargo Features and Conditional Compilation
// This exercise demonstrates:
// - Using cargo features
// - Conditional compilation
// - Feature dependencies
fn exercise1() {
    println!("Exercise 1: Cargo Features and Conditional Compilation");
    println!("------------------------------------------------");
    println!("TODO: Design a library with feature flags\n");

    // In this exercise, you'll design a library with feature flags.
    // This would typically be done in a separate crate, but we'll
    // describe the process here.

    println!("Library: data_processor");
    println!("Purpose: Process data in various formats");
    println!("\nFeatures to implement:");
    println!("  1. json: JSON format support");
    println!("  2. xml: XML format support");
    println!("  3. csv: CSV format support");
    println!("  4. async: Asynchronous processing");
    println!("  5. logging: Logging support");

    println!("\nCargo.toml configuration:");
    println!("[package]");
    println!("name = \"data_processor\"");
    println!("version = \"0.1.0\"");
    println!("edition = \"2021\"");
    println!("");
    println!("[dependencies]");
    println!("serde = \"1.0\"");
    println!("log = {{ version = \"0.4\", optional = true }}");
    println!("");
    println!("[dependencies.serde_json]");
    println!("version = \"1.0\"");
    println!("optional = true");
    println!("");
    println!("[dependencies.quick-xml]");
    println!("version = \"0.22\"");
    println!("optional = true");
    println!("");
    println!("[dependencies.csv]");
    println!("version = \"1.1\"");
    println!("optional = true");
    println!("");
    println!("[dependencies.tokio]");
    println!("version = \"1.0\"");
    println!("optional = true");
    println!("features = [\"full\"]");
    println!("");
    println!("[features]");
    println!("default = [\"json\"]");
    println!("json = [\"serde_json\"]");
    println!("xml = [\"quick-xml\"]");
    println!("csv = [\"dep:csv\"]");
    println!("async = [\"tokio\"]");
    println!("logging = [\"log\"]");
    println!("all = [\"json\", \"xml\", \"csv\", \"async\", \"logging\"]");

    println!("\nExample implementation with conditional compilation:");
    println!("#[cfg(feature = \"json\")]");
    println!("pub fn parse_json(data: &str) -> Result<Value, Error> {");
    println!("    serde_json::from_str(data)");
    println!("}");
    println!("");
    println!("#[cfg(feature = \"xml\")]");
    println!("pub fn parse_xml(data: &str) -> Result<Value, Error> {");
    println!("    // XML parsing implementation");
    println!("}");
    println!("");
    println!("#[cfg(feature = \"async\")]");
    println!("pub async fn process_async(data: &str) -> Result<Value, Error> {");
    println!("    // Async processing implementation");
    println!("}");

    println!("\nUsage example:");
    println!("// With default features");
    println!("data_processor = \"0.1.0\"");
    println!("");
    println!("// With specific features");
    println!(
        "data_processor = {{ version = \"0.1.0\", features = [\"json\", \"csv\", \"logging\"] }}"
    );
    println!("");
    println!("// With all features");
    println!("data_processor = {{ version = \"0.1.0\", features = [\"all\"] }}");
    println!("");
    println!("// Without default features");
    println!("data_processor = {{ version = \"0.1.0\", default-features = false, features = [\"xml\"] }}");
}

// Exercise 2: Workspace Management
// This exercise demonstrates:
// - Setting up a workspace
// - Managing dependencies across packages
// - Using workspace inheritance
fn exercise2() {
    println!("\nExercise 2: Workspace Management");
    println!("----------------------------");
    println!("TODO: Design a workspace for a web application\n");

    // In this exercise, you'll design a workspace for a web application.
    // This would typically be done in a separate directory, but we'll
    // describe the process here.

    println!("Workspace: web_app");
    println!("Purpose: A web application with multiple components");
    println!("\nPackages to include:");
    println!("  1. web_app_core: Core functionality");
    println!("  2. web_app_api: API server");
    println!("  3. web_app_cli: Command-line interface");
    println!("  4. web_app_common: Shared utilities");

    println!("\nWorkspace Cargo.toml:");
    println!("[workspace]");
    println!("members = [");
    println!("    \"web_app_core\",");
    println!("    \"web_app_api\",");
    println!("    \"web_app_cli\",");
    println!("    \"web_app_common\",");
    println!("]");
    println!("");
    println!("[workspace.package]");
    println!("version = \"0.1.0\"");
    println!("authors = [\"Your Name <your.email@example.com>\"]");
    println!("edition = \"2021\"");
    println!("");
    println!("[workspace.dependencies]");
    println!("serde = {{ version = \"1.0\", features = [\"derive\"] }}");
    println!("tokio = {{ version = \"1.0\", features = [\"full\"] }}");
    println!("log = \"0.4\"");
    println!("env_logger = \"0.9\"");
    println!("web_app_common = {{ path = \"./web_app_common\" }}");
    println!("web_app_core = {{ path = \"./web_app_core\" }}");

    println!("\nPackage Cargo.toml (web_app_api):");
    println!("[package]");
    println!("name = \"web_app_api\"");
    println!("version.workspace = true");
    println!("authors.workspace = true");
    println!("edition.workspace = true");
    println!("");
    println!("[dependencies]");
    println!("serde.workspace = true");
    println!("tokio.workspace = true");
    println!("log.workspace = true");
    println!("env_logger.workspace = true");
    println!("web_app_common.workspace = true");
    println!("web_app_core.workspace = true");
    println!("axum = \"0.5\"");

    println!("\nCommands to run:");
    println!("  cargo build --workspace: Build all packages");
    println!("  cargo test --workspace: Test all packages");
    println!("  cargo run -p web_app_api: Run the API server");
    println!("  cargo run -p web_app_cli: Run the CLI");
}

// Exercise 3: Cross Compilation
// This exercise demonstrates:
// - Setting up cross compilation
// - Target-specific configuration
// - Building for different platforms
fn exercise3() {
    println!("\nExercise 3: Cross Compilation");
    println!("-------------------------");
    println!("TODO: Set up cross compilation for a CLI tool\n");

    // In this exercise, you'll set up cross compilation for a CLI tool.
    // This would typically involve actual commands, but we'll describe
    // the process here.

    println!("CLI Tool: file_analyzer");
    println!("Purpose: Analyze files and provide statistics");
    println!("\nTarget platforms:");
    println!("  1. x86_64-unknown-linux-gnu (64-bit Linux)");
    println!("  2. x86_64-pc-windows-msvc (64-bit Windows)");
    println!("  3. aarch64-apple-darwin (64-bit macOS on ARM)");

    println!("\nSteps for cross compilation:");
    println!("1. Install Rust targets:");
    println!("   rustup target add x86_64-unknown-linux-gnu");
    println!("   rustup target add x86_64-pc-windows-msvc");
    println!("   rustup target add aarch64-apple-darwin");
    println!("");
    println!("2. Install cross-compilation tools:");
    println!("   # For Linux target on non-Linux host");
    println!("   apt-get install gcc-x86-64-linux-gnu");
    println!("   # For Windows target on non-Windows host");
    println!("   apt-get install gcc-mingw-w64");
    println!("");
    println!("3. Configure .cargo/config.toml:");
    println!("[target.x86_64-unknown-linux-gnu]");
    println!("linker = \"x86_64-linux-gnu-gcc\"");
    println!("");
    println!("[target.x86_64-pc-windows-msvc]");
    println!("linker = \"x86_64-w64-mingw32-gcc\"");
    println!("ar = \"x86_64-w64-mingw32-ar\"");
    println!("");
    println!("[target.aarch64-apple-darwin]");
    println!("linker = \"aarch64-apple-darwin-clang\"");
    println!("ar = \"aarch64-apple-darwin-ar\"");
    println!("");
    println!("4. Build for each target:");
    println!("   cargo build --target x86_64-unknown-linux-gnu");
    println!("   cargo build --target x86_64-pc-windows-msvc");
    println!("   cargo build --target aarch64-apple-darwin");
    println!("");
    println!("5. Create release builds:");
    println!("   cargo build --release --target x86_64-unknown-linux-gnu");
    println!("   cargo build --release --target x86_64-pc-windows-msvc");
    println!("   cargo build --release --target aarch64-apple-darwin");

    println!("\nHandling platform-specific code:");
    println!("#[cfg(target_os = \"windows\")]");
    println!("fn get_file_path() -> &'static str {");
    println!("    \"C:\\\\path\\\\to\\\\file.txt\"");
    println!("}");
    println!("");
    println!("#[cfg(target_os = \"linux\")]");
    println!("fn get_file_path() -> &'static str {");
    println!("    \"/path/to/file.txt\"");
    println!("}");
    println!("");
    println!("#[cfg(target_os = \"macos\")]");
    println!("fn get_file_path() -> &'static str {");
    println!("    \"/path/to/file.txt\"");
    println!("}");
}

// Exercise 4: Profiling and Optimization
// This exercise demonstrates:
// - Profiling Rust code
// - Identifying bottlenecks
// - Applying optimizations
fn exercise4() {
    println!("\nExercise 4: Profiling and Optimization");
    println!("----------------------------------");
    println!("TODO: Profile and optimize a data processing function\n");

    // In this exercise, you'll profile and optimize a data processing function.
    // This would typically involve actual code and profiling, but we'll describe
    // the process here.

    println!("Function to optimize: process_large_dataset");
    println!("Purpose: Process a large dataset and compute statistics");

    println!("\nOriginal implementation (pseudocode):");
    println!("fn process_large_dataset(data: &[u64]) -> Statistics {");
    println!("    let mut result = Statistics::new();");
    println!("    for &value in data {");
    println!("        result.update(value);");
    println!("    }");
    println!("    result");
    println!("}");

    println!("\nProfiling steps:");
    println!("1. Build with debug info:");
    println!("   cargo build --release");
    println!("");
    println!("2. Run with perf (Linux):");
    println!("   perf record --call-graph dwarf ./target/release/my_program");
    println!("   perf report");
    println!("");
    println!("3. Generate flamegraph:");
    println!("   cargo install flamegraph");
    println!("   cargo flamegraph");
    println!("");
    println!("4. Use criterion for benchmarking:");
    println!("   [dev-dependencies]");
    println!("   criterion = \"0.3\"");
    println!("");
    println!("   [[bench]]");
    println!("   name = \"benchmark\"");
    println!("   harness = false");

    println!("\nOptimization techniques:");
    println!("1. Parallelization with rayon:");
    println!("fn process_large_dataset(data: &[u64]) -> Statistics {");
    println!("    data.par_iter()");
    println!("        .map(|&value| {");
    println!("            let mut local_stats = Statistics::new();");
    println!("            local_stats.update(value);");
    println!("            local_stats");
    println!("        })");
    println!("        .reduce(|| Statistics::new(), |a, b| a.combine(b))");
    println!("}");
    println!("");
    println!("2. SIMD optimization (for numeric operations):");
    println!("#[cfg(target_arch = \"x86_64\")]");
    println!("fn sum_array(data: &[f32]) -> f32 {");
    println!("    if is_x86_feature_detected!(\"avx2\") {");
    println!("        unsafe { sum_array_avx2(data) }");
    println!("    } else {");
    println!("        data.iter().sum()");
    println!("    }");
    println!("}");
    println!("");
    println!("3. Memory optimization:");
    println!("- Use appropriate data structures");
    println!("- Minimize allocations");
    println!("- Consider custom allocators for specific use cases");
    println!("- Use appropriate sized types");

    println!("\nBenchmarking results (hypothetical):");
    println!("Original implementation: 1000ms");
    println!("Parallelized implementation: 250ms");
    println!("SIMD-optimized implementation: 100ms");
    println!("Memory-optimized implementation: 80ms");
}

// Exercise 5: Publishing a Crate
// This exercise demonstrates:
// - Preparing a crate for publication
// - Documentation best practices
// - Versioning and maintenance
fn exercise5() {
    println!("\nExercise 5: Publishing a Crate");
    println!("---------------------------");
    println!("TODO: Prepare a crate for publication\n");

    // In this exercise, you'll prepare a crate for publication.
    // This would typically involve actual code and commands, but we'll describe
    // the process here.

    println!("Crate: config_loader");
    println!("Purpose: Load configuration from various sources");

    println!("\nSteps for publication:");
    println!("1. Choose a name and check availability:");
    println!("   cargo search config_loader");
    println!("");
    println!("2. Set up metadata in Cargo.toml:");
    println!("[package]");
    println!("name = \"config_loader\"");
    println!("version = \"0.1.0\"");
    println!("authors = [\"Your Name <your.email@example.com>\"]");
    println!("edition = \"2021\"");
    println!("description = \"Load configuration from various sources\"");
    println!("license = \"MIT OR Apache-2.0\"");
    println!("repository = \"https://github.com/yourusername/config_loader\"");
    println!("documentation = \"https://docs.rs/config_loader\"");
    println!("keywords = [\"config\", \"configuration\", \"loader\"]");
    println!("categories = [\"config\"]");
    println!("readme = \"README.md\"");
    println!("");
    println!("3. Write comprehensive documentation:");
    println!("//! # Config Loader");
    println!("//!");
    println!("//! A library for loading configuration from various sources.");
    println!("//!");
    println!("//! ## Features");
    println!("//!");
    println!("//! - Load from files (JSON, YAML, TOML)");
    println!("//! - Load from environment variables");
    println!("//! - Load from command-line arguments");
    println!("//! - Merge configuration from multiple sources");
    println!("//!");
    println!("//! ## Examples");
    println!("//!");
    println!("//! ```");
    println!("//! use config_loader::ConfigLoader;");
    println!("//!");
    println!("//! let config = ConfigLoader::new()");
    println!("//!     .add_file(\"config.toml\")");
    println!("//!     .add_env_prefix(\"APP_\")");
    println!("//!     .load()");
    println!("//!     .unwrap();");
    println!("//!");
    println!("//! let server_port = config.get::<u16>(\"server.port\").unwrap_or(8080);");
    println!("//! ```");
    println!("");
    println!("4. Write a good README.md:");
    println!("# Config Loader");
    println!("");
    println!("A library for loading configuration from various sources.");
    println!("");
    println!("## Features");
    println!("");
    println!("- Load from files (JSON, YAML, TOML)");
    println!("- Load from environment variables");
    println!("- Load from command-line arguments");
    println!("- Merge configuration from multiple sources");
    println!("");
    println!("## Usage");
    println!("");
    println!("```rust");
    println!("use config_loader::ConfigLoader;");
    println!("");
    println!("let config = ConfigLoader::new()");
    println!("    .add_file(\"config.toml\")");
    println!("    .add_env_prefix(\"APP_\")");
    println!("    .load()");
    println!("    .unwrap();");
    println!("");
    println!("let server_port = config.get::<u16>(\"server.port\").unwrap_or(8080);");
    println!("```");
    println!("");
    println!("5. Test thoroughly:");
    println!("   cargo test");
    println!("");
    println!("6. Check for common issues:");
    println!("   cargo clippy");
    println!("   cargo fmt");
    println!("");
    println!("7. Verify the package:");
    println!("   cargo package --list");
    println!("");
    println!("8. Publish the crate:");
    println!("   cargo login");
    println!("   cargo publish");

    println!("\nMaintenance best practices:");
    println!("1. Follow Semantic Versioning");
    println!("2. Keep a CHANGELOG.md");
    println!("3. Respond to issues and pull requests");
    println!("4. Write migration guides for breaking changes");
    println!("5. Set up CI for automated testing");
    println!("6. Use cargo-release for version management");
}

/* Example Solutions:
 * For complete solutions to these exercises, refer to the documentation
 * or the companion guide. Try solving them yourself first!
 */
