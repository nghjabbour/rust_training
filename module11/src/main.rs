// Module 11: Rust Tooling and Ecosystem
// This module explores Rust's tooling and ecosystem:
// - Cargo features in depth
// - Workspace management
// - Cross compilation
// - Profiling and optimization
// - Debug symbols and DWARF
// - Popular crates and their internals
// - Contributing to the Rust ecosystem

mod problems;

fn main() {
    // Run the practice problems
    problems::run_exercises();

    // ===============================
    // 1. Cargo Features in Depth
    // ===============================
    println!("\n1. Cargo Features in Depth:");
    println!("------------------------");
    // Cargo is Rust's package manager and build tool
    // - Dependency management
    // - Compilation
    // - Testing
    // - Documentation generation
    // - Publishing

    println!("Key cargo commands:");
    println!("  cargo new: Create a new project");
    println!("  cargo build: Compile the project");
    println!("  cargo run: Compile and run the project");
    println!("  cargo test: Run tests");
    println!("  cargo doc: Generate documentation");
    println!("  cargo publish: Publish to crates.io");

    println!("\nCargo.toml sections:");
    println!("  [package]: Project metadata");
    println!("  [dependencies]: Project dependencies");
    println!("  [dev-dependencies]: Development dependencies");
    println!("  [build-dependencies]: Build script dependencies");
    println!("  [features]: Conditional compilation features");
    println!("  [workspace]: Workspace configuration");

    // ===============================
    // 2. Workspace Management
    // ===============================
    println!("\n2. Workspace Management:");
    println!("---------------------");
    // Workspaces allow managing multiple related packages
    // - Shared dependencies
    // - Coordinated versioning
    // - Efficient builds

    println!("Workspace structure:");
    println!("  my_workspace/");
    println!("  ├── Cargo.toml");
    println!("  ├── package1/");
    println!("  │   ├── Cargo.toml");
    println!("  │   └── src/");
    println!("  └── package2/");
    println!("      ├── Cargo.toml");
    println!("      └── src/");

    println!("\nWorkspace Cargo.toml:");
    println!("  [workspace]");
    println!("  members = [");
    println!("      \"package1\",");
    println!("      \"package2\",");
    println!("  ]");

    println!("\nWorkspace commands:");
    println!("  cargo build --workspace: Build all packages");
    println!("  cargo test --workspace: Test all packages");
    println!("  cargo run -p package1: Run specific package");

    // ===============================
    // 3. Cross Compilation
    // ===============================
    println!("\n3. Cross Compilation:");
    println!("------------------");
    // Cross compilation allows building for different target platforms
    // - Different operating systems
    // - Different architectures
    // - Embedded systems

    println!("Cross compilation steps:");
    println!("  1. Install target: rustup target add <target>");
    println!("  2. Install toolchain: apt-get install gcc-arm-linux-gnueabihf");
    println!("  3. Configure .cargo/config.toml");
    println!("  4. Build: cargo build --target <target>");

    println!("\nCommon targets:");
    println!("  x86_64-unknown-linux-gnu: 64-bit Linux");
    println!("  x86_64-pc-windows-msvc: 64-bit Windows");
    println!("  aarch64-apple-darwin: 64-bit macOS on ARM");
    println!("  wasm32-unknown-unknown: WebAssembly");
    println!("  arm-unknown-linux-gnueabihf: ARM Linux");

    println!("\n.cargo/config.toml example:");
    println!("  [target.aarch64-unknown-linux-gnu]");
    println!("  linker = \"aarch64-linux-gnu-gcc\"");

    // ===============================
    // 4. Profiling and Optimization
    // ===============================
    println!("\n4. Profiling and Optimization:");
    println!("---------------------------");
    // Profiling helps identify performance bottlenecks
    // Optimization improves code performance

    println!("Profiling tools:");
    println!("  perf: Linux performance analyzer");
    println!("  flamegraph: Visualization of CPU usage");
    println!("  valgrind: Memory usage analysis");
    println!("  criterion: Benchmarking library");

    println!("\nOptimization levels:");
    println!("  -O0 (dev): No optimizations");
    println!("  -O1: Basic optimizations");
    println!("  -O2: More optimizations");
    println!("  -O3 (release): All optimizations");
    println!("  -Os: Optimize for size");
    println!("  -Oz: Optimize for size aggressively");

    println!("\nOptimization techniques:");
    println!("  1. Use release mode: cargo build --release");
    println!("  2. Profile to find bottlenecks");
    println!("  3. Use appropriate data structures");
    println!("  4. Minimize allocations");
    println!("  5. Consider SIMD for numeric code");
    println!("  6. Use parallel processing where appropriate");

    // ===============================
    // 5. Debug Symbols and DWARF
    // ===============================
    println!("\n5. Debug Symbols and DWARF:");
    println!("-------------------------");
    // Debug symbols provide information for debugging
    // DWARF is a debugging data format

    println!("Debug symbols:");
    println!("  - Enable with debug = true in Cargo.toml");
    println!("  - Control with debug-assertions flag");
    println!("  - Split debug info with -Csplit-debuginfo");

    println!("\nDWARF format:");
    println!("  - Debug With Arbitrary Record Format");
    println!("  - Contains type information");
    println!("  - Contains source line information");
    println!("  - Contains variable locations");

    println!("\nDebugging tools:");
    println!("  - gdb: GNU Debugger");
    println!("  - lldb: LLVM Debugger");
    println!("  - rust-gdb: GDB with Rust extensions");
    println!("  - rust-lldb: LLDB with Rust extensions");

    // ===============================
    // 6. Popular Crates and Their Internals
    // ===============================
    println!("\n6. Popular Crates and Their Internals:");
    println!("----------------------------------");
    // Understanding popular crates and how they work

    println!("serde:");
    println!("  - Serialization/deserialization framework");
    println!("  - Uses procedural macros for code generation");
    println!("  - Supports custom formats via traits");

    println!("\ntokio:");
    println!("  - Asynchronous runtime");
    println!("  - Event-driven architecture");
    println!("  - Work-stealing scheduler");
    println!("  - I/O and timer subsystems");

    println!("\nrayon:");
    println!("  - Data parallelism library");
    println!("  - Work-stealing thread pool");
    println!("  - Parallel iterators");
    println!("  - Join operation for recursive parallelism");

    println!("\nclap:");
    println!("  - Command-line argument parser");
    println!("  - Builder pattern API");
    println!("  - Derive macros for struct-based API");
    println!("  - Automatic help generation");

    // ===============================
    // 7. Contributing to the Rust Ecosystem
    // ===============================
    println!("\n7. Contributing to the Rust Ecosystem:");
    println!("----------------------------------");
    // How to contribute to the Rust ecosystem

    println!("Ways to contribute:");
    println!("  - Report bugs and issues");
    println!("  - Submit pull requests");
    println!("  - Write documentation");
    println!("  - Answer questions on forums");
    println!("  - Create and maintain crates");

    println!("\nContributing to Rust itself:");
    println!("  - Find issues labeled 'E-easy' or 'E-mentor'");
    println!("  - Join working groups");
    println!("  - Participate in RFC process");
    println!("  - Help with compiler development");

    println!("\nPublishing crates:");
    println!("  1. Choose a name and check availability");
    println!("  2. Set up metadata in Cargo.toml");
    println!("  3. Write documentation and tests");
    println!("  4. Publish with cargo publish");
    println!("  5. Maintain and respond to issues");

    // ===============================
    // 8. Real-World Project Setup
    // ===============================
    println!("\n8. Real-World Project Setup:");
    println!("-------------------------");
    // Best practices for setting up real-world Rust projects

    println!("Project structure:");
    println!("  my_project/");
    println!("  ├── Cargo.toml");
    println!("  ├── src/");
    println!("  │   ├── main.rs");
    println!("  │   ├── lib.rs");
    println!("  │   └── bin/");
    println!("  ├── tests/");
    println!("  ├── benches/");
    println!("  ├── examples/");
    println!("  └── docs/");

    println!("\nContinuous Integration:");
    println!("  - GitHub Actions");
    println!("  - Travis CI");
    println!("  - CircleCI");
    println!("  - Run tests, lints, and formatting checks");

    println!("\nDocumentation:");
    println!("  - README.md with examples");
    println!("  - API documentation with examples");
    println!("  - CONTRIBUTING.md for contributors");
    println!("  - CHANGELOG.md for version history");

    println!("\nVersioning:");
    println!("  - Follow Semantic Versioning");
    println!("  - Document breaking changes");
    println!("  - Use cargo-release for publishing");
}

// Notes on Rust Tooling and Ecosystem:
//
// 1. Cargo
//    - Package manager and build tool
//    - Dependency management
//    - Build configuration
//    - Testing, benchmarking, documentation
//    - Publishing to crates.io
//
// 2. Workspaces
//    - Multiple related packages
//    - Shared dependencies
//    - Coordinated versioning
//    - Efficient builds
//
// 3. Cross Compilation
//    - Building for different platforms
//    - Target-specific configuration
//    - Toolchain management
//    - Embedded development
//
// 4. Profiling and Optimization
//    - Performance analysis
//    - Optimization levels
//    - Benchmarking
//    - Memory usage analysis
//
// 5. Ecosystem
//    - crates.io: Central package registry
//    - docs.rs: Documentation hosting
//    - lib.rs: Alternative package index
//    - GitHub: Source code hosting
//    - Rust users forum: Community support

// Try experimenting with these concepts:
// 1. Set up a workspace with multiple packages
// 2. Cross-compile a simple program for a different platform
// 3. Profile a program to identify performance bottlenecks
// 4. Publish a small crate to crates.io
// 5. Set up CI for a Rust project
