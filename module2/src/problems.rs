// Module 2: Practice Problems
// These exercises focus on Rust's ownership system and memory management.
// Each problem is designed to deepen your understanding of how Rust manages memory
// and why its approach leads to memory-safe, efficient code.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn run_exercises() {
    println!("Module 2 Exercises - Ownership and Memory Management");
    println!("==============================================\n");

    exercise1();
    exercise2();
    exercise3();
    exercise4();
    exercise5();
}

// Exercise 1: Memory Layout and Ownership
// This exercise demonstrates:
// - Stack vs heap allocation
// - Memory layout of different types
// - Ownership transfer
// - Drop behavior
fn exercise1() {
    println!("Exercise 1: Memory Layout and Ownership");
    println!("----------------------------------");
    println!("TODO: Implement the memory_layout_examples function\n");

    // Custom types with different memory characteristics
    #[derive(Debug)]
    struct StackStruct {
        x: i32,
        y: i32,
        z: bool,
    }

    #[derive(Debug)]
    struct HeapStruct {
        data: Vec<i32>,
        name: String,
    }

    // Implement this function to demonstrate memory layout
    fn memory_layout_examples() {
        // TODO: Create instances of both structs and demonstrate:
        // 1. Size and alignment differences
        // 2. Ownership transfer behavior
        // 3. Drop behavior
        // 4. Clone vs Copy semantics
        unimplemented!("Implement memory_layout_examples");
    }

    // Test your implementation:
    // memory_layout_examples();
}

// Exercise 2: Advanced Borrowing
// This exercise demonstrates:
// - Complex borrowing patterns
// - Lifetime relationships
// - Mutable and immutable borrows
// - Borrowing rules and scope
fn exercise2() {
    println!("\nExercise 2: Advanced Borrowing");
    println!("---------------------------");
    println!("TODO: Implement the DataTracker struct and its methods\n");

    // A struct that tracks data and its access patterns
    struct DataTracker<T> {
        data: T,
        access_count: usize,
        modifications: Vec<String>,
    }

    impl<T: std::fmt::Debug> DataTracker<T> {
        // Create new tracker
        fn new(data: T) -> Self {
            unimplemented!("Implement new");
        }

        // Borrow data immutably and record access
        fn get_data(&mut self) -> &T {
            unimplemented!("Implement get_data");
        }

        // Modify data and record modification
        fn modify_data<F>(&mut self, modifier: F, description: &str)
        where
            F: FnOnce(&mut T),
        {
            unimplemented!("Implement modify_data");
        }

        // Get access statistics
        fn get_stats(&self) -> (usize, &[String]) {
            unimplemented!("Implement get_stats");
        }
    }

    // Test your implementation:
    // let mut tracker = DataTracker::new(vec![1, 2, 3]);
    // println!("Initial data: {:?}", tracker.get_data());
    // tracker.modify_data(|v| v.push(4), "Added 4");
    // let (accesses, mods) = tracker.get_stats();
    // println!("Accesses: {}, Modifications: {:?}", accesses, mods);
}

// Exercise 3: Resource Management
// This exercise demonstrates:
// - RAII pattern
// - Custom Drop implementation
// - Resource cleanup
// - Error handling with resources
fn exercise3() {
    println!("\nExercise 3: Resource Management");
    println!("----------------------------");
    println!("TODO: Implement the ResourcePool struct and its methods\n");

    // A generic resource pool that manages cleanup
    struct ResourcePool<T> {
        resources: Vec<T>,
        max_size: usize,
    }

    // A resource that requires cleanup
    struct Resource {
        id: usize,
        data: Vec<u8>,
    }

    impl Resource {
        fn new(id: usize) -> Self {
            unimplemented!("Implement Resource::new");
        }
    }

    impl Drop for Resource {
        fn drop(&mut self) {
            // Simulate resource cleanup
            println!("Cleaning up resource {}", self.id);
        }
    }

    impl<T> ResourcePool<T> {
        // Initialize the pool
        fn new(max_size: usize) -> Self {
            unimplemented!("Implement ResourcePool::new");
        }

        // Add resource to pool, return error if pool is full
        fn add_resource(&mut self, resource: T) -> Result<(), String> {
            unimplemented!("Implement add_resource");
        }

        // Remove and return resource if available
        fn take_resource(&mut self) -> Option<T> {
            unimplemented!("Implement take_resource");
        }
    }

    // Test your implementation:
    // let mut pool = ResourcePool::new(2);
    // pool.add_resource(Resource::new(1)).unwrap();
    // pool.add_resource(Resource::new(2)).unwrap();
    // assert!(pool.add_resource(Resource::new(3)).is_err()); // Pool is full
}

// Exercise 4: Shared Ownership
// This exercise demonstrates:
// - Reference counting with Rc<T>
// - Interior mutability with RefCell<T>
// - Circular references
// - Memory leak prevention
fn exercise4() {
    println!("\nExercise 4: Shared Ownership");
    println!("--------------------------");
    println!("TODO: Implement the Graph struct and its methods\n");

    // A node in a graph that can have multiple parents and children
    type NodeHandle = Rc<RefCell<Node>>;

    struct Node {
        id: usize,
        data: String,
        children: Vec<NodeHandle>,
        // Weak references to parents to prevent reference cycles
        parents: Vec<std::rc::Weak<RefCell<Node>>>,
    }

    struct Graph {
        nodes: HashMap<usize, NodeHandle>,
    }

    impl Node {
        fn new(id: usize, data: String) -> Self {
            unimplemented!("Implement Node::new");
        }
    }

    impl Graph {
        fn new() -> Self {
            unimplemented!("Implement Graph::new");
        }

        // Add a node to the graph
        fn add_node(&mut self, id: usize, data: String) -> NodeHandle {
            unimplemented!("Implement add_node");
        }

        // Add an edge between nodes
        fn add_edge(&mut self, from_id: usize, to_id: usize) -> Result<(), String> {
            unimplemented!("Implement add_edge");
        }

        // Get all ancestors of a node
        fn get_ancestors(&self, id: usize) -> Vec<usize> {
            unimplemented!("Implement get_ancestors");
        }
    }

    // Test your implementation:
    // let mut graph = Graph::new();
    // let node1 = graph.add_node(1, "One".to_string());
    // let node2 = graph.add_node(2, "Two".to_string());
    // graph.add_edge(1, 2).unwrap();
    // println!("Ancestors of 2: {:?}", graph.get_ancestors(2));
}

// Exercise 5: Safe Abstractions
// This exercise demonstrates:
// - Building safe abstractions
// - Ownership and borrowing in APIs
// - Error handling
// - Type-state programming
fn exercise5() {
    println!("\nExercise 5: Safe Abstractions");
    println!("--------------------------");
    println!("TODO: Implement the Connection types and methods\n");

    // Type-state programming example: Connection handling
    struct DisconnectedConnection;
    struct ConnectedConnection {
        buffer: Vec<u8>,
    }
    struct FailedConnection {
        error: String,
    }

    // Connection that can be in different states
    enum Connection {
        Disconnected(DisconnectedConnection),
        Connected(ConnectedConnection),
        Failed(FailedConnection),
    }

    impl DisconnectedConnection {
        // Attempt to connect
        fn connect(self) -> Connection {
            unimplemented!("Implement connect");
        }
    }

    impl ConnectedConnection {
        // Send data
        fn send_data(&mut self, data: &[u8]) -> Result<(), String> {
            unimplemented!("Implement send_data");
        }

        // Disconnect gracefully
        fn disconnect(self) -> DisconnectedConnection {
            unimplemented!("Implement disconnect");
        }
    }

    impl Connection {
        // Create new connection in disconnected state
        fn new() -> Self {
            Connection::Disconnected(DisconnectedConnection)
        }

        // Handle the connection based on its state
        fn handle(&mut self) {
            unimplemented!("Implement handle");
        }
    }

    // Test your implementation:
    // let mut conn = Connection::new();
    // conn.handle();
}

/* Example Solutions (Try to solve the exercises before looking at these!)

// Exercise 1 Solution:
fn memory_layout_examples() {
    let stack_struct = StackStruct { x: 1, y: 2, z: true };
    let heap_struct = HeapStruct {
        data: vec![1, 2, 3],
        name: String::from("example"),
    };

    println!("Stack struct size: {} bytes", std::mem::size_of::<StackStruct>());
    println!("Heap struct size: {} bytes", std::mem::size_of::<HeapStruct>());

    // Move semantics
    let moved_stack = stack_struct; // Creates a copy
    println!("Original still valid: {:?}", stack_struct);

    let moved_heap = heap_struct; // Moves ownership
    // println!("Original invalid: {:?}", heap_struct); // Would not compile
}

// Exercise 2 Solution:
impl<T: std::fmt::Debug> DataTracker<T> {
    fn new(data: T) -> Self {
        DataTracker {
            data,
            access_count: 0,
            modifications: Vec::new(),
        }
    }

    fn get_data(&mut self) -> &T {
        self.access_count += 1;
        &self.data
    }

    fn modify_data<F>(&mut self, modifier: F, description: &str)
    where
        F: FnOnce(&mut T),
    {
        modifier(&mut self.data);
        self.modifications.push(description.to_string());
    }

    fn get_stats(&self) -> (usize, &[String]) {
        (self.access_count, &self.modifications)
    }
}

// Exercise 3 Solution:
impl<T> ResourcePool<T> {
    fn new(max_size: usize) -> Self {
        ResourcePool {
            resources: Vec::new(),
            max_size,
        }
    }

    fn add_resource(&mut self, resource: T) -> Result<(), String> {
        if self.resources.len() >= self.max_size {
            Err("Pool is full".to_string())
        } else {
            self.resources.push(resource);
            Ok(())
        }
    }

    fn take_resource(&mut self) -> Option<T> {
        self.resources.pop()
    }
}

// Exercise 4 Solution:
impl Graph {
    fn add_node(&mut self, id: usize, data: String) -> NodeHandle {
        let node = Rc::new(RefCell::new(Node::new(id, data)));
        self.nodes.insert(id, Rc::clone(&node));
        node
    }

    fn add_edge(&mut self, from_id: usize, to_id: usize) -> Result<(), String> {
        let from_node = self.nodes.get(&from_id).ok_or("From node not found")?;
        let to_node = self.nodes.get(&to_id).ok_or("To node not found")?;

        from_node.borrow_mut().children.push(Rc::clone(to_node));
        to_node.borrow_mut().parents.push(Rc::downgrade(from_node));

        Ok(())
    }
}

// Exercise 5 Solution:
impl ConnectedConnection {
    fn send_data(&mut self, data: &[u8]) -> Result<(), String> {
        self.buffer.extend_from_slice(data);
        Ok(())
    }

    fn disconnect(self) -> DisconnectedConnection {
        DisconnectedConnection
    }
}

impl Connection {
    fn handle(&mut self) {
        match self {
            Connection::Disconnected(conn) => {
                *self = conn.connect();
            }
            Connection::Connected(conn) => {
                if let Err(e) = conn.send_data(&[1, 2, 3]) {
                    *self = Connection::Failed(FailedConnection {
                        error: e,
                    });
                }
            }
            Connection::Failed(_) => {
                // Handle failure state
            }
        }
    }
}
*/
