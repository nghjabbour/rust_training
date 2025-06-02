// Module 8: Smart Pointers and Interior Mutability
// This module explores Rust's smart pointer types and interior mutability patterns:
// - Box<T> and heap allocation
// - Rc<T> and reference counting
// - RefCell<T> and interior mutability
// - Weak<T> and breaking reference cycles
// - Custom smart pointer implementation
// - Understanding Drop and destructors
// - Memory leaks and how to prevent them

mod problems;

use std::cell::{Cell, RefCell};
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

fn main() {
    // Run the practice problems
    problems::run_exercises();

    // ===============================
    // 1. Box<T> and Heap Allocation
    // ===============================
    println!("\n1. Box<T> and Heap Allocation:");
    println!("---------------------------");
    // Box<T> is a smart pointer that stores data on the heap
    // - Useful for recursive types
    // - Moving ownership of large data
    // - Ensuring fixed-size types

    // Basic usage
    let boxed_value = Box::new(42);
    println!("Boxed value: {}", boxed_value);

    // Recursive types with Box
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("List: {:?}", list);

    // Box internals
    println!("\nBox internals:");
    println!("  - Single pointer to heap-allocated data");
    println!("  - Zero runtime overhead compared to raw pointers");
    println!("  - Memory is freed when Box goes out of scope");
    println!(
        "  - Size of Box<T>: {} bytes",
        std::mem::size_of::<Box<i32>>()
    );

    // ===============================
    // 2. Rc<T> and Reference Counting
    // ===============================
    println!("\n2. Rc<T> and Reference Counting:");
    println!("-----------------------------");
    // Rc<T> allows multiple ownership of the same data
    // - Reference counted
    // - Immutable access only
    // - Not thread-safe

    // Creating shared ownership
    let original = Rc::new(String::from("Hello"));
    println!("Original reference count: {}", Rc::strong_count(&original));

    {
        let clone1 = Rc::clone(&original);
        println!("After clone1 - count: {}", Rc::strong_count(&original));

        let clone2 = Rc::clone(&original);
        println!("After clone2 - count: {}", Rc::strong_count(&original));

        // All clones point to the same data
        println!("All point to same data: {} {} {}", original, clone1, clone2);
    } // clone1 and clone2 go out of scope here

    println!("After inner scope - count: {}", Rc::strong_count(&original));

    // Rc internals
    println!("\nRc internals:");
    println!("  - Contains pointer to heap-allocated control block");
    println!("  - Control block contains the data and reference counts");
    println!("  - clone() increments the strong count");
    println!("  - drop() decrements the strong count");
    println!("  - Data is freed when strong count reaches zero");

    // ===============================
    // 3. RefCell<T> and Interior Mutability
    // ===============================
    println!("\n3. RefCell<T> and Interior Mutability:");
    println!("----------------------------------");
    // RefCell<T> allows mutable borrows checked at runtime
    // - Interior mutability pattern
    // - Enforces borrowing rules at runtime
    // - Not thread-safe

    // Basic usage
    let data = RefCell::new(42);
    println!("Original value: {:?}", data);

    // Mutable borrow
    *data.borrow_mut() = 100;
    println!("After mutation: {:?}", data);

    // Multiple immutable borrows
    let borrow1 = data.borrow();
    let borrow2 = data.borrow();
    println!("Multiple immutable borrows: {} {}", borrow1, borrow2);
    drop(borrow1);
    drop(borrow2);

    // RefCell with Rc for shared mutable data
    #[derive(Debug)]
    struct SharedData {
        value: RefCell<i32>,
    }

    let shared = Rc::new(SharedData {
        value: RefCell::new(42),
    });

    let shared1 = Rc::clone(&shared);
    let shared2 = Rc::clone(&shared);

    // Modify through different Rc handles
    *shared1.value.borrow_mut() += 10;
    *shared2.value.borrow_mut() += 5;

    println!("Final shared value: {}", shared.value.borrow());

    // RefCell internals
    println!("\nRefCell internals:");
    println!("  - Contains the data and borrow flags");
    println!("  - borrow() and borrow_mut() check and update flags");
    println!("  - Panics if borrowing rules are violated");
    println!("  - Borrows are RAII guards that update flags when dropped");

    // ===============================
    // 4. Cell<T> for Copy Types
    // ===============================
    println!("\n4. Cell<T> for Copy Types:");
    println!("------------------------");
    // Cell<T> is a simpler form of interior mutability
    // - No borrowing, just replacement
    // - More efficient for Copy types
    // - Not thread-safe

    let cell = Cell::new(10);
    println!("Original cell value: {}", cell.get());

    cell.set(20);
    println!("New cell value: {}", cell.get());

    // Cell vs RefCell
    println!("\nCell vs RefCell:");
    println!("  - Cell: get() returns a copy, no borrowing");
    println!("  - RefCell: borrow() returns a reference, tracks borrows");
    println!("  - Cell is more efficient but only works with Copy types");
    println!("  - RefCell works with any type but has runtime overhead");

    // ===============================
    // 5. Weak<T> and Reference Cycles
    // ===============================
    println!("\n5. Weak<T> and Reference Cycles:");
    println!("-----------------------------");
    // Weak<T> is a non-owning reference to Rc<T> data
    // - Prevents reference cycles
    // - Must be upgraded to access data
    // - Data may be gone when upgrading

    // Parent-child relationship with cycles
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    // Create parent node
    let parent = Rc::new(Node {
        value: 1,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // Create child node with reference to parent
    let child = Rc::new(Node {
        value: 2,
        parent: RefCell::new(Rc::downgrade(&parent)), // Weak reference
        children: RefCell::new(vec![]),
    });

    // Add child to parent
    parent.children.borrow_mut().push(Rc::clone(&child));

    // Access relationships
    println!("Parent value: {}", parent.value);
    println!(
        "Child's parent value: {}",
        child.parent.borrow().upgrade().unwrap().value
    );
    println!(
        "Parent's child value: {}",
        parent.children.borrow()[0].value
    );

    // Reference counts
    println!("\nReference counts:");
    println!("Parent strong count: {}", Rc::strong_count(&parent));
    println!("Parent weak count: {}", Rc::weak_count(&parent));
    println!("Child strong count: {}", Rc::strong_count(&child));
    println!("Child weak count: {}", Rc::weak_count(&child));

    // ===============================
    // 6. Custom Smart Pointers
    // ===============================
    println!("\n6. Custom Smart Pointers:");
    println!("----------------------");
    // Implementing your own smart pointer
    // - Deref and DerefMut traits
    // - Drop trait for cleanup
    // - Custom behavior

    // A simple smart pointer
    struct MyBox<T>(Box<T>);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(Box::new(x))
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> DerefMut for MyBox<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl<T> Drop for MyBox<T> {
        fn drop(&mut self) {
            println!("Dropping MyBox!");
        }
    }

    // Using our custom smart pointer
    let my_box = MyBox::new(String::from("Hello"));
    println!("MyBox content: {}", *my_box);
    println!("MyBox length: {}", my_box.len()); // Deref coercion

    // ===============================
    // 7. Drop and Destructors
    // ===============================
    println!("\n7. Drop and Destructors:");
    println!("---------------------");
    // The Drop trait provides a way to run code when a value goes out of scope
    // - Similar to destructors in other languages
    // - Used for cleanup
    // - Runs in reverse order of creation

    struct CustomResource {
        name: String,
    }

    impl Drop for CustomResource {
        fn drop(&mut self) {
            println!("Cleaning up resource: {}", self.name);
        }
    }

    // Resources are dropped in reverse order
    let resource1 = CustomResource {
        name: String::from("Resource 1"),
    };
    let resource2 = CustomResource {
        name: String::from("Resource 2"),
    };

    println!(
        "Resources created: {} and {}",
        resource1.name, resource2.name
    );
    println!("End of scope approaching...");
    // resource2 will be dropped first, then resource1

    // ===============================
    // 8. Preventing Memory Leaks
    // ===============================
    println!("\n8. Preventing Memory Leaks:");
    println!("------------------------");
    // Even with garbage collection, memory leaks can occur
    // - Reference cycles with Rc<T>
    // - Forgetting to clean up resources
    // - Leaking memory intentionally with std::mem::forget

    // Example of a potential reference cycle
    println!("Reference cycles can cause memory leaks");
    println!("Use Weak<T> to break cycles");
    println!("Use drop() to explicitly clean up resources");
    println!("Be careful with std::mem::forget()");

    // Intentional leak (uncomment to see)
    // let leaked = Box::new(String::from("I'm leaked!"));
    // std::mem::forget(leaked); // Memory is never freed
}

// Notes on Smart Pointers and Interior Mutability:
//
// 1. Smart Pointer Types
//    - Box<T>: Single ownership, heap allocation
//    - Rc<T>: Multiple ownership, reference counting
//    - Arc<T>: Thread-safe reference counting
//    - RefCell<T>: Interior mutability, runtime borrow checking
//    - Cell<T>: Interior mutability for Copy types
//    - Weak<T>: Non-owning reference to Rc<T> data
//
// 2. Memory Management
//    - Box<T>: Freed when it goes out of scope
//    - Rc<T>: Freed when last strong reference is dropped
//    - Weak<T>: Does not prevent cleanup
//    - RefCell<T>: Enforces borrowing rules at runtime
//
// 3. Use Cases
//    - Box<T>: Recursive types, trait objects, large data
//    - Rc<T>: Shared ownership, tree structures
//    - RefCell<T>: Mutable data behind immutable interface
//    - Weak<T>: Breaking reference cycles
//
// 4. Performance Characteristics
//    - Box<T>: Zero runtime overhead compared to raw pointers
//    - Rc<T>: Small overhead for reference counting
//    - RefCell<T>: Runtime borrow checking overhead
//    - Cell<T>: Minimal overhead for interior mutability
//
// 5. Safety Guarantees
//    - Box<T>: Memory safety, no leaks
//    - Rc<T>: Memory safety, potential cycles
//    - RefCell<T>: Runtime borrow checking, potential panics
//    - Weak<T>: Safe access to potentially dropped data

// Try experimenting with these concepts:
// 1. Create a recursive data structure with Box<T>
// 2. Implement a shared cache with Rc<T> and RefCell<T>
// 3. Build a tree structure with parent-child relationships using Weak<T>
// 4. Create a custom smart pointer with Deref and Drop
// 5. Explore memory leak scenarios and how to prevent them
