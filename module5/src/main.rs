// Module 5: Traits and Generics Deep Dive
// This module provides an in-depth exploration of Rust's trait system:
// - Static vs dynamic dispatch
// - Trait objects and virtual method tables
// - Advanced trait patterns
// - Auto traits and marker traits
// - Zero-cost abstractions
// - Comparison with other languages' polymorphism

mod problems;

use std::fmt::{self, Debug, Display};
use std::ops::{Add, Deref, DerefMut};

fn main() {
    // Run the practice problems
    problems::run_exercises();

    // ===============================
    // 1. Trait System Fundamentals
    // ===============================
    println!("\n1. Trait System Fundamentals:");
    println!("--------------------------");
    // Traits are Rust's primary abstraction mechanism
    // Unlike interfaces in other languages:
    // - Zero runtime cost (static dispatch by default)
    // - Can be implemented for existing types (orphan rules)
    // - Support default implementations
    // - Can have associated types and constants

    // Trait for category information (not used in dynamic dispatch)
    trait Category {
        // Associated constant
        const CATEGORY: &'static str = "Unknown";
    }

    // Basic trait with default implementation (dyn compatible)
    trait Describable {
        // Required method
        fn name(&self) -> &str;

        // Optional method with default implementation
        fn describe(&self) -> String {
            format!("This is a {}", self.name())
        }
    }

    // Implementing for existing type
    struct Point {
        x: f64,
        y: f64,
    }

    impl Category for Point {
        // Override the default
        const CATEGORY: &'static str = "Geometry";
    }

    impl Describable for Point {
        fn name(&self) -> &str {
            "Point"
        }
    }

    let point = Point { x: 1.0, y: 2.0 };
    println!("Name: {}", point.name());
    println!("Description: {}", point.describe());
    println!("Category: {}", <Point as Category>::CATEGORY);

    // ===============================
    // 2. Static vs Dynamic Dispatch
    // ===============================
    println!("\n2. Static vs Dynamic Dispatch:");
    println!("---------------------------");
    // Rust supports both static (compile-time) and dynamic (runtime) dispatch
    // Static dispatch: Zero cost, monomorphization
    // Dynamic dispatch: Runtime cost, flexibility

    // Static dispatch (default)
    fn print_static<T: Describable>(item: &T) {
        println!("Static: {}", item.describe());
    }

    // Dynamic dispatch (trait objects)
    fn print_dynamic(item: &dyn Describable) {
        println!("Dynamic: {}", item.describe());
    }

    // Compare sizes
    println!(
        "Size of concrete Point: {} bytes",
        std::mem::size_of::<Point>()
    );
    println!(
        "Size of trait object: {} bytes",
        std::mem::size_of::<&dyn Describable>()
    );
    println!(
        "Size of generic fn: {} bytes",
        std::mem::size_of::<fn(&Point)>()
    );

    // ===============================
    // 3. Advanced Trait Patterns
    // ===============================
    println!("\n3. Advanced Trait Patterns:");
    println!("------------------------");

    // Associated types vs generic parameters
    trait Container {
        type Item; // Associated type
        fn get(&self) -> Option<&Self::Item>;
        fn insert(&mut self, item: Self::Item);
    }

    trait GenericContainer<T> {
        // Generic parameter
        fn get(&self) -> Option<&T>;
        fn insert(&mut self, item: T);
    }

    // Associated types are better when each type should only implement
    // the trait once, while generic parameters allow multiple implementations

    // Conditional trait implementations
    #[derive(Debug)]
    struct Wrapper<T>(T);

    impl<T: Display> Display for Wrapper<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Wrapper({})", self.0)
        }
    }

    // ===============================
    // 4. Trait Objects and vtables
    // ===============================
    println!("\n4. Trait Objects and vtables:");
    println!("---------------------------");
    // Trait objects consist of:
    // 1. Pointer to the data
    // 2. Pointer to the vtable (virtual method table)

    // Creating a trait object
    let shapes: Vec<Box<dyn Describable>> = vec![
        Box::new(Point { x: 0.0, y: 0.0 }),
        Box::new(Circle { radius: 1.0 }),
    ];

    // Each call goes through the vtable
    for shape in &shapes {
        println!("{}", shape.describe());
    }

    // ===============================
    // 5. Auto Traits and Markers
    // ===============================
    println!("\n5. Auto Traits and Markers:");
    println!("--------------------------");
    // Auto traits are automatically implemented
    // Marker traits have no methods

    // Custom marker trait
    trait Serializable {}

    // Auto trait (implemented automatically if all fields implement Send)
    unsafe trait MySend {}
    unsafe impl<T: MySend> MySend for Box<T> {}

    // ===============================
    // 6. Advanced Type System Features
    // ===============================
    println!("\n6. Advanced Type System Features:");
    println!("------------------------------");

    // Higher-ranked trait bounds
    fn transform_slice<T>(slice: &[T]) -> &[T]
    where
        for<'a> &'a T: Display,
    {
        slice
    }

    // Associated type constructors
    trait Collection {
        type Item;
        type Iterator<'a>: Iterator<Item = &'a Self::Item>
        where
            Self: 'a;

        fn iter<'a>(&'a self) -> Self::Iterator<'a>;
    }

    // ===============================
    // 7. Zero-Cost Abstractions
    // ===============================
    println!("\n7. Zero-Cost Abstractions:");
    println!("--------------------------");

    // Custom smart pointer
    struct SmartPtr<T> {
        data: Box<T>,
    }

    impl<T> Deref for SmartPtr<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    impl<T> DerefMut for SmartPtr<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.data
        }
    }

    // ===============================
    // 8. Operator Overloading
    // ===============================
    println!("\n8. Operator Overloading:");
    println!("----------------------");

    // Complex number implementation
    #[derive(Debug, Copy, Clone)]
    struct Complex {
        re: f64,
        im: f64,
    }

    impl Add for Complex {
        type Output = Complex;

        fn add(self, other: Complex) -> Complex {
            Complex {
                re: self.re + other.re,
                im: self.im + other.im,
            }
        }
    }

    // Supporting structs/impls
    struct Circle {
        radius: f64,
    }

    impl Category for Circle {
        const CATEGORY: &'static str = "Geometry";
    }

    impl Describable for Circle {
        fn name(&self) -> &str {
            "Circle"
        }
    }
}

// Notes on Rust's Trait System:
//
// 1. Design Philosophy
//    - Zero-cost abstractions
//    - Static dispatch by default
//    - Runtime polymorphism when needed
//    - Type safety without runtime overhead
//
// 2. Comparison with Other Languages
//    - Java/C# interfaces: Runtime dispatch only
//    - C++ templates: Compile-time only, no constraints
//    - Haskell typeclasses: Similar but with different rules
//    - Go interfaces: Structural typing vs nominal typing
//
// 3. Performance Characteristics
//    - Static dispatch: No runtime overhead
//    - Dynamic dispatch: Two pointer indirections
//    - Monomorphization: Code size vs runtime performance
//    - Zero-sized types optimization
//
// 4. Best Practices
//    - Prefer static dispatch for small types
//    - Use dynamic dispatch for large types or plugins
//    - Consider associated types for single implementations
//    - Use generic parameters for multiple implementations
//
// 5. Advanced Features
//    - Specialization (unstable)
//    - Associated type constructors
//    - Higher-ranked trait bounds
//    - Auto traits
//    - Marker traits

// Try experimenting with these concepts:
// 1. Create traits with associated types
// 2. Implement dynamic dispatch
// 3. Create custom operators
// 4. Use higher-ranked trait bounds
// 5. Create zero-cost abstractions
