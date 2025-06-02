// Module 10: Advanced Features and Unsafe Rust
// This module explores Rust's advanced features and unsafe code:
// - Procedural macros
// - Attribute macros
// - Derive macros
// - Raw pointers and unsafe blocks
// - FFI and C interop
// - SIMD and platform intrinsics
// - Custom allocators
// - Advanced type system features

mod problems;

use std::alloc::{GlobalAlloc, Layout};
use std::ptr;

fn main() {
    // Run the practice problems
    problems::run_exercises();

    // ===============================
    // 1. Procedural Macros
    // ===============================
    println!("\n1. Procedural Macros:");
    println!("------------------");
    // Procedural macros are functions that operate on Rust syntax at compile time
    // - Function-like macros: custom!(...)
    // - Derive macros: #[derive(CustomDerive)]
    // - Attribute macros: #[custom_attribute]

    println!("Procedural macros are defined in separate crates");
    println!("They operate on TokenStream inputs and outputs");
    println!("Common uses include:");
    println!("  - Custom derive implementations");
    println!("  - Domain-specific languages (DSLs)");
    println!("  - Code generation");
    println!("  - Compile-time validation");

    // Example of a function-like procedural macro (conceptual)
    println!("\nExample function-like macro:");
    println!("  sql!(SELECT * FROM users WHERE id = 1)");
    println!("  // Expands to code that performs the SQL query");

    // Example of a derive macro (conceptual)
    println!("\nExample derive macro:");
    println!("  #[derive(Serialize, Deserialize)]");
    println!("  struct User { name: String, email: String }");

    // Example of an attribute macro (conceptual)
    println!("\nExample attribute macro:");
    println!("  #[route(GET, \"/users/{id}\")]");
    println!("  fn get_user(id: u32) -> User { ... }");

    // ===============================
    // 2. Raw Pointers and Unsafe Blocks
    // ===============================
    println!("\n2. Raw Pointers and Unsafe Blocks:");
    println!("------------------------------");
    // Raw pointers provide direct memory access
    // - *const T: Immutable raw pointer
    // - *mut T: Mutable raw pointer
    // - No safety guarantees
    // - Must be used in unsafe blocks

    // Creating raw pointers (safe)
    let mut value = 42;
    let ptr = &value as *const i32; // Immutable raw pointer
    let mut_ptr = &mut value as *mut i32; // Mutable raw pointer

    println!("Raw pointers: {:?}, {:?}", ptr, mut_ptr);

    // Dereferencing raw pointers (unsafe)
    unsafe {
        println!("Value through raw pointer: {}", *ptr);
        *mut_ptr = 100;
        println!("Modified value: {}", value);
    }

    // Null pointer
    let null_ptr: *const i32 = ptr::null();
    println!("Null pointer: {:?}", null_ptr);

    // Pointer arithmetic (unsafe)
    let array = [1, 2, 3, 4, 5];
    let array_ptr = array.as_ptr();

    unsafe {
        println!("Array[0]: {}", *array_ptr);
        println!("Array[2]: {}", *array_ptr.add(2));
    }

    // ===============================
    // 3. FFI and C Interop
    // ===============================
    println!("\n3. FFI and C Interop:");
    println!("------------------");
    // Foreign Function Interface allows calling C code from Rust
    // - extern "C" blocks declare external functions
    // - #[repr(C)] ensures C-compatible memory layout
    // - libc crate provides C standard library bindings

    // Declaring external C functions
    extern "C" {
        fn abs(input: i32) -> i32;
        fn sqrt(input: f64) -> f64;
    }

    // Calling C functions (unsafe)
    unsafe {
        println!("abs(-42) = {}", abs(-42));
        println!("sqrt(16.0) = {}", sqrt(16.0));
    }

    // Exposing Rust functions to C
    #[no_mangle]
    pub extern "C" fn rust_function(x: i32) -> i32 {
        x * 2
    }

    println!("\nExposing Rust to C:");
    println!("  #[no_mangle]");
    println!("  pub extern \"C\" fn rust_function(x: i32) -> i32 {{");
    println!("      x * 2");
    println!("  }}");

    // C-compatible structs
    #[repr(C)]
    struct CCompatibleStruct {
        x: i32,
        y: i32,
    }

    println!("\nC-compatible struct:");
    println!("  #[repr(C)]");
    println!("  struct CCompatibleStruct {{");
    println!("      x: i32,");
    println!("      y: i32,");
    println!("  }}");

    // ===============================
    // 4. SIMD and Platform Intrinsics
    // ===============================
    println!("\n4. SIMD and Platform Intrinsics:");
    println!("-----------------------------");
    // SIMD (Single Instruction, Multiple Data) allows parallel operations
    // - std::arch module provides platform-specific intrinsics
    // - Requires target_feature or runtime detection

    println!("SIMD operations process multiple data elements in parallel");
    println!("Platform intrinsics provide access to CPU-specific instructions");
    println!("Example SIMD code (conceptual):");
    println!("  #[cfg(target_arch = \"x86_64\")]");
    println!("  use std::arch::x86_64::*;");
    println!("  unsafe {{");
    println!("      let a = _mm256_set1_ps(1.0);");
    println!("      let b = _mm256_set1_ps(2.0);");
    println!("      let c = _mm256_add_ps(a, b);");
    println!("  }}");

    // ===============================
    // 5. Custom Allocators
    // ===============================
    println!("\n5. Custom Allocators:");
    println!("-------------------");
    // Custom allocators allow controlling memory allocation
    // - Implement the GlobalAlloc trait
    // - #[global_allocator] attribute sets the global allocator

    // A simple (but terrible) allocator that just uses the system allocator
    struct MyAllocator;

    unsafe impl GlobalAlloc for MyAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            // In a real allocator, you would implement proper allocation logic
            // This example just forwards to the system allocator
            std::alloc::System.alloc(layout)
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            std::alloc::System.dealloc(ptr, layout)
        }
    }

    // Note: We don't actually set this as the global allocator
    // #[global_allocator]
    // static ALLOCATOR: MyAllocator = MyAllocator;

    println!("Custom allocators allow controlling memory allocation");
    println!("Example custom allocator (conceptual):");
    println!("  struct PoolAllocator {{ ... }}");
    println!("  unsafe impl GlobalAlloc for PoolAllocator {{ ... }}");
    println!("  #[global_allocator]");
    println!("  static ALLOCATOR: PoolAllocator = PoolAllocator::new();");

    // ===============================
    // 6. Advanced Type System Features
    // ===============================
    println!("\n6. Advanced Type System Features:");
    println!("-----------------------------");
    // Rust's type system has many advanced features
    // - Associated type constructors
    // - Higher-ranked trait bounds
    // - Existential types
    // - Const generics

    // Associated type constructors (GATs)
    println!("Generic Associated Types (GATs):");
    println!("  trait Collection {{");
    println!("      type Item<'a> where Self: 'a;");
    println!("      fn get(&self, index: usize) -> Option<Self::Item<'_>>;");
    println!("  }}");

    // Higher-ranked trait bounds
    println!("\nHigher-ranked trait bounds:");
    println!("  fn for_all_lifetimes<T>(value: T)");
    println!("  where for<'a> T: Trait<'a> {{ ... }}");

    // Const generics
    println!("\nConst generics:");
    println!("  struct Array<T, const N: usize> {{");
    println!("      data: [T; N],");
    println!("  }}");

    // ===============================
    // 7. Unsafe Traits and Implementations
    // ===============================
    println!("\n7. Unsafe Traits and Implementations:");
    println!("---------------------------------");
    // Unsafe traits and impls indicate additional safety requirements
    // - unsafe trait: Implementing the trait requires upholding safety invariants
    // - unsafe impl: Implementation upholds the required safety invariants

    // Unsafe trait example
    unsafe trait UnsafeTrait {
        unsafe fn unsafe_method(&self);
    }

    struct UnsafeStruct;

    unsafe impl UnsafeTrait for UnsafeStruct {
        unsafe fn unsafe_method(&self) {
            println!("This method requires unsafe because it has safety requirements");
        }
    }

    println!("Unsafe traits indicate additional safety requirements");
    println!("Examples of unsafe traits in std:");
    println!("  - Send: Types that can be transferred between threads");
    println!("  - Sync: Types that can be shared between threads");
    println!("  - GlobalAlloc: Types that can be used as global allocators");

    // ===============================
    // 8. Building Safe Abstractions
    // ===============================
    println!("\n8. Building Safe Abstractions:");
    println!("---------------------------");
    // Safe abstractions encapsulate unsafe code
    // - Enforce invariants at the API boundary
    // - Document safety requirements
    // - Use the type system to prevent misuse

    // Example: A safe wrapper around a raw pointer
    struct SafeWrapper<T> {
        ptr: *mut T,
        len: usize,
    }

    impl<T> SafeWrapper<T> {
        fn new(value: T) -> Self {
            let ptr = Box::into_raw(Box::new(value));
            SafeWrapper { ptr, len: 1 }
        }

        fn get(&self) -> &T {
            unsafe { &*self.ptr }
        }

        fn get_mut(&mut self) -> &mut T {
            unsafe { &mut *self.ptr }
        }
    }

    impl<T> Drop for SafeWrapper<T> {
        fn drop(&mut self) {
            unsafe {
                Box::from_raw(self.ptr);
            }
        }
    }

    // Using the safe wrapper
    let mut wrapper = SafeWrapper::new(42);
    println!("Value: {}", wrapper.get());
    *wrapper.get_mut() = 100;
    println!("Modified value: {}", wrapper.get());

    println!("\nPrinciples for safe abstractions:");
    println!("  - Enforce invariants at the API boundary");
    println!("  - Document safety requirements");
    println!("  - Use the type system to prevent misuse");
    println!("  - Minimize the scope of unsafe blocks");
    println!("  - Test thoroughly, including edge cases");
}

// Notes on Advanced Features and Unsafe Rust:
//
// 1. When to Use Unsafe
//    - Implementing fundamental data structures
//    - Performance-critical code
//    - FFI and interoperability
//    - Low-level systems programming
//    - SIMD and platform-specific optimizations
//
// 2. Safety Guidelines
//    - Minimize unsafe code
//    - Encapsulate unsafe code in safe abstractions
//    - Document safety requirements and invariants
//    - Test thoroughly
//    - Consider formal verification
//
// 3. Common Unsafe Patterns
//    - Raw pointer manipulation
//    - FFI and C interoperability
//    - Custom memory management
//    - SIMD and intrinsics
//    - Mutable static variables
//
// 4. Procedural Macros
//    - Function-like macros: custom!(...)
//    - Derive macros: #[derive(CustomDerive)]
//    - Attribute macros: #[custom_attribute]
//    - Operate on TokenStream inputs and outputs
//
// 5. Advanced Type System Features
//    - Generic associated types (GATs)
//    - Higher-ranked trait bounds
//    - Const generics
//    - Existential types
//    - Negative trait bounds (unstable)

// Try experimenting with these concepts:
// 1. Create a safe abstraction over unsafe code
// 2. Implement a simple procedural macro
// 3. Use FFI to call a C library
// 4. Experiment with SIMD operations
// 5. Create a custom allocator for specific use cases
