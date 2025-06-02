# Module 10: Advanced Features and Unsafe Rust

## Overview
This module explores Rust's advanced features and the unsafe subset of the language. While Rust's safety guarantees are one of its strongest selling points, there are situations where you need more control and are willing to take responsibility for memory safety. This module covers advanced type system features, FFI (Foreign Function Interface), and how to use unsafe Rust responsibly.

## Learning Objectives
- Understand advanced type system features like associated types and type aliases
- Learn about macros and metaprogramming in Rust
- Master the Foreign Function Interface (FFI) for calling C code from Rust
- Understand when and how to use unsafe Rust responsibly
- Implement raw pointers and unsafe functions
- Learn about advanced lifetime patterns

## How to Use This Module
1. Start by reading through the explanations and examples in `src/main.rs`
2. Run the code to see the concepts in action:
   ```
   cd module10
   cargo run
   ```
3. Study the exercises in `src/problems.rs`
4. Try to understand each exercise before looking at the solutions
5. Experiment by modifying the code to deepen your understanding

## Exercises
This module contains exercises that focus on:
1. **Advanced Type System Features**: Working with associated types and type aliases
2. **Macros**: Creating and using declarative and procedural macros
3. **FFI**: Calling C functions from Rust and vice versa
4. **Unsafe Blocks**: Writing unsafe code safely
5. **Raw Pointers**: Working with *const T and *mut T pointers

## Prerequisites
- Completion of Modules 1-9
- Solid understanding of Rust's type system and memory model
- Familiarity with C programming is helpful but not required for FFI sections

## Next Steps
After completing this module, proceed to Module 11 to learn about Rust tooling and ecosystem.