// Module 8: Practice Problems
// These exercises focus on smart pointers and interior mutability in Rust.
// Each problem includes detailed explanations of the underlying concepts
// and how they enable complex memory management patterns.

use std::cell::{Cell, RefCell};
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

pub fn run_exercises() {
    println!("Module 8 Exercises - Smart Pointers and Interior Mutability");
    println!("=====================================================\n");

    exercise1();
    exercise2();
    exercise3();
    exercise4();
    exercise5();
}

// Exercise 1: Custom Smart Pointer
// This exercise demonstrates:
// - Implementing Deref and DerefMut traits
// - Understanding smart pointer behavior
// - Custom Drop implementation
fn exercise1() {
    println!("Exercise 1: Custom Smart Pointer");
    println!("--------------------------");
    println!("TODO: Implement the SmartString struct and its traits\n");

    // A smart pointer for strings with additional functionality
    struct SmartString {
        // TODO: Implement the fields needed for a smart string
    }

    impl SmartString {
        // Create a new SmartString
        fn new(s: &str) -> Self {
            unimplemented!("Implement SmartString::new");
        }

        // Get the length of the string
        fn len(&self) -> usize {
            unimplemented!("Implement len");
        }

        // Check if the string is empty
        fn is_empty(&self) -> bool {
            unimplemented!("Implement is_empty");
        }

        // Convert to uppercase
        fn to_uppercase(&self) -> SmartString {
            unimplemented!("Implement to_uppercase");
        }
    }

    impl Deref for SmartString {
        type Target = str;

        fn deref(&self) -> &Self::Target {
            unimplemented!("Implement deref");
        }
    }

    impl DerefMut for SmartString {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unimplemented!("Implement deref_mut");
        }
    }

    impl Drop for SmartString {
        fn drop(&mut self) {
            unimplemented!("Implement drop");
        }
    }

    // Test your implementation:
    // let mut smart = SmartString::new("Hello, world!");
    // println!("Smart string: {}", smart);
    // println!("Length: {}", smart.len());
    // println!("Is empty: {}", smart.is_empty());
    //
    // // Test deref coercion
    // println!("Contains 'world': {}", smart.contains("world"));
    //
    // // Test to_uppercase
    // let upper = smart.to_uppercase();
    // println!("Uppercase: {}", upper);
    //
    // // Test DerefMut
    // let bytes = unsafe { smart.as_bytes_mut() };
    // bytes[0] = b'h';
    // println!("Modified: {}", smart);
}

// Exercise 2: Recursive Data Structure
// This exercise demonstrates:
// - Using Box<T> for recursive types
// - Tree structures with owned nodes
// - Traversal algorithms
fn exercise2() {
    println!("\nExercise 2: Recursive Data Structure");
    println!("-------------------------------");
    println!("TODO: Implement the BinaryTree struct and its methods\n");

    // A binary tree with owned nodes
    enum BinaryTree<T> {
        Leaf,
        Node(Box<TreeNode<T>>),
    }

    struct TreeNode<T> {
        value: T,
        left: BinaryTree<T>,
        right: BinaryTree<T>,
    }

    impl<T: Ord + std::fmt::Debug> BinaryTree<T> {
        // Create a new empty tree
        fn new() -> Self {
            unimplemented!("Implement BinaryTree::new");
        }

        // Insert a value into the tree
        fn insert(&mut self, value: T) {
            unimplemented!("Implement insert");
        }

        // Check if the tree contains a value
        fn contains(&self, value: &T) -> bool {
            unimplemented!("Implement contains");
        }

        // Print the tree in-order
        fn print_in_order(&self) {
            unimplemented!("Implement print_in_order");
        }

        // Calculate the height of the tree
        fn height(&self) -> usize {
            unimplemented!("Implement height");
        }
    }

    // Test your implementation:
    // let mut tree = BinaryTree::new();
    // tree.insert(5);
    // tree.insert(3);
    // tree.insert(7);
    // tree.insert(2);
    // tree.insert(4);
    //
    // println!("Tree height: {}", tree.height());
    // println!("Contains 4: {}", tree.contains(&4));
    // println!("Contains 6: {}", tree.contains(&6));
    //
    // println!("In-order traversal:");
    // tree.print_in_order();
}

// Exercise 3: Shared Cache
// This exercise demonstrates:
// - Using Rc<T> for shared ownership
// - Using RefCell<T> for interior mutability
// - Implementing a cache with shared access
fn exercise3() {
    println!("\nExercise 3: Shared Cache");
    println!("-------------------");
    println!("TODO: Implement the SharedCache struct and its methods\n");

    // A cache that can be shared between multiple owners
    struct SharedCache<K, V> {
        // TODO: Implement the fields needed for a shared cache
    }

    impl<K, V> SharedCache<K, V>
    where
        K: std::cmp::Eq + std::hash::Hash + Clone,
        V: Clone,
    {
        // Create a new shared cache
        fn new() -> Rc<Self> {
            unimplemented!("Implement SharedCache::new");
        }

        // Get a value from the cache
        fn get(&self, key: &K) -> Option<V> {
            unimplemented!("Implement get");
        }

        // Insert a value into the cache
        fn insert(&self, key: K, value: V) {
            unimplemented!("Implement insert");
        }

        // Get the number of items in the cache
        fn len(&self) -> usize {
            unimplemented!("Implement len");
        }

        // Check if the cache is empty
        fn is_empty(&self) -> bool {
            unimplemented!("Implement is_empty");
        }
    }

    // Test your implementation:
    // let cache = SharedCache::new();
    // let cache_clone = Rc::clone(&cache);
    //
    // // Insert from original
    // cache.insert("key1".to_string(), "value1".to_string());
    // cache.insert("key2".to_string(), "value2".to_string());
    //
    // // Get from clone
    // println!("key1: {:?}", cache_clone.get(&"key1".to_string()));
    // println!("key2: {:?}", cache_clone.get(&"key2".to_string()));
    //
    // // Insert from clone
    // cache_clone.insert("key3".to_string(), "value3".to_string());
    //
    // // Get from original
    // println!("key3: {:?}", cache.get(&"key3".to_string()));
    // println!("Cache size: {}", cache.len());
}

// Exercise 4: Object Graph with Weak References
// This exercise demonstrates:
// - Using Weak<T> to prevent reference cycles
// - Building complex object graphs
// - Parent-child relationships
fn exercise4() {
    println!("\nExercise 4: Object Graph with Weak References");
    println!("----------------------------------------");
    println!("TODO: Implement the Component and GameObject structs\n");

    // A component in a game object system
    struct Component {
        // TODO: Implement the fields needed for a component
    }

    // A game object that contains components
    struct GameObject {
        // TODO: Implement the fields needed for a game object
    }

    impl Component {
        // Create a new component
        fn new(name: &str, game_object: &Rc<GameObject>) -> Rc<Self> {
            unimplemented!("Implement Component::new");
        }

        // Get the name of the component
        fn name(&self) -> &str {
            unimplemented!("Implement name");
        }

        // Get the parent game object
        fn game_object(&self) -> Option<Rc<GameObject>> {
            unimplemented!("Implement game_object");
        }
    }

    impl GameObject {
        // Create a new game object
        fn new(name: &str) -> Rc<Self> {
            unimplemented!("Implement GameObject::new");
        }

        // Get the name of the game object
        fn name(&self) -> &str {
            unimplemented!("Implement name");
        }

        // Add a component to the game object
        fn add_component(&self, component: Rc<Component>) {
            unimplemented!("Implement add_component");
        }

        // Get a component by name
        fn get_component(&self, name: &str) -> Option<Rc<Component>> {
            unimplemented!("Implement get_component");
        }

        // Get all components
        fn get_components(&self) -> Vec<Rc<Component>> {
            unimplemented!("Implement get_components");
        }
    }

    // Test your implementation:
    // let game_object = GameObject::new("Player");
    // let transform = Component::new("Transform", &game_object);
    // let renderer = Component::new("Renderer", &game_object);
    //
    // game_object.add_component(transform);
    // game_object.add_component(renderer);
    //
    // println!("Game object: {}", game_object.name());
    // println!("Components:");
    // for component in game_object.get_components() {
    //     println!("  - {}", component.name());
    //     println!("    Parent: {}", component.game_object().unwrap().name());
    // }
    //
    // // Check reference counts
    // println!("Game object strong count: {}", Rc::strong_count(&game_object));
    // println!("Game object weak count: {}", Rc::weak_count(&game_object));
}

// Exercise 5: Memory Pool
// This exercise demonstrates:
// - Custom memory management
// - Reusing allocated memory
// - Safe abstractions over raw memory
fn exercise5() {
    println!("\nExercise 5: Memory Pool");
    println!("------------------");
    println!("TODO: Implement the MemoryPool struct and its methods\n");

    // A memory pool that allocates and reuses fixed-size blocks
    struct MemoryPool<T> {
        // TODO: Implement the fields needed for a memory pool
    }

    impl<T> MemoryPool<T> {
        // Create a new memory pool with the given capacity
        fn new(capacity: usize) -> Self {
            unimplemented!("Implement MemoryPool::new");
        }

        // Allocate a new object in the pool
        fn allocate(&mut self, value: T) -> Option<usize> {
            unimplemented!("Implement allocate");
        }

        // Deallocate an object from the pool
        fn deallocate(&mut self, index: usize) -> bool {
            unimplemented!("Implement deallocate");
        }

        // Get a reference to an object in the pool
        fn get(&self, index: usize) -> Option<&T> {
            unimplemented!("Implement get");
        }

        // Get a mutable reference to an object in the pool
        fn get_mut(&mut self, index: usize) -> Option<&mut T> {
            unimplemented!("Implement get_mut");
        }

        // Get the number of allocated objects
        fn allocated_count(&self) -> usize {
            unimplemented!("Implement allocated_count");
        }
    }

    impl<T> Drop for MemoryPool<T> {
        fn drop(&mut self) {
            unimplemented!("Implement drop");
        }
    }

    // Test your implementation:
    // let mut pool = MemoryPool::new(10);
    //
    // // Allocate objects
    // let idx1 = pool.allocate(String::from("Object 1")).unwrap();
    // let idx2 = pool.allocate(String::from("Object 2")).unwrap();
    // let idx3 = pool.allocate(String::from("Object 3")).unwrap();
    //
    // // Get objects
    // println!("Object 1: {}", pool.get(idx1).unwrap());
    // println!("Object 2: {}", pool.get(idx2).unwrap());
    // println!("Object 3: {}", pool.get(idx3).unwrap());
    //
    // // Modify an object
    // if let Some(obj) = pool.get_mut(idx2) {
    //     obj.push_str(" (modified)");
    // }
    // println!("Modified object 2: {}", pool.get(idx2).unwrap());
    //
    // // Deallocate an object
    // pool.deallocate(idx1);
    // println!("After deallocation, allocated count: {}", pool.allocated_count());
    //
    // // Reallocate
    // let idx4 = pool.allocate(String::from("Object 4")).unwrap();
    // println!("New object 4: {}", pool.get(idx4).unwrap());
}

/* Example Solutions (Try to solve the exercises before looking at these!)

// Exercise 1 Solution:
struct SmartString {
    data: String,
    access_count: Cell<usize>,
}

impl SmartString {
    fn new(s: &str) -> Self {
        SmartString {
            data: s.to_string(),
            access_count: Cell::new(0),
        }
    }

    fn len(&self) -> usize {
        self.access_count.set(self.access_count.get() + 1);
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.access_count.set(self.access_count.get() + 1);
        self.data.is_empty()
    }

    fn to_uppercase(&self) -> SmartString {
        self.access_count.set(self.access_count.get() + 1);
        SmartString::new(&self.data.to_uppercase())
    }
}

impl Deref for SmartString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.access_count.set(self.access_count.get() + 1);
        &self.data
    }
}

impl DerefMut for SmartString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.access_count.set(self.access_count.get() + 1);
        &mut self.data
    }
}

impl Drop for SmartString {
    fn drop(&mut self) {
        println!("Dropping SmartString with content '{}', accessed {} times",
            self.data, self.access_count.get());
    }
}

// Exercise 2 Solution:
impl<T: Ord + std::fmt::Debug> BinaryTree<T> {
    fn new() -> Self {
        BinaryTree::Leaf
    }

    fn insert(&mut self, value: T) {
        match self {
            BinaryTree::Leaf => {
                *self = BinaryTree::Node(Box::new(TreeNode {
                    value,
                    left: BinaryTree::Leaf,
                    right: BinaryTree::Leaf,
                }));
            }
            BinaryTree::Node(ref mut node) => {
                if value < node.value {
                    node.left.insert(value);
                } else {
                    node.right.insert(value);
                }
            }
        }
    }

    fn contains(&self, value: &T) -> bool {
        match self {
            BinaryTree::Leaf => false,
            BinaryTree::Node(node) => {
                if &node.value == value {
                    true
                } else if value < &node.value {
                    node.left.contains(value)
                } else {
                    node.right.contains(value)
                }
            }
        }
    }

    fn print_in_order(&self) {
        match self {
            BinaryTree::Leaf => {}
            BinaryTree::Node(node) => {
                node.left.print_in_order();
                println!("{:?}", node.value);
                node.right.print_in_order();
            }
        }
    }

    fn height(&self) -> usize {
        match self {
            BinaryTree::Leaf => 0,
            BinaryTree::Node(node) => {
                1 + std::cmp::max(node.left.height(), node.right.height())
            }
        }
    }
}

// Exercise 3 Solution:
use std::collections::HashMap;

struct SharedCache<K, V> {
    data: RefCell<HashMap<K, V>>,
}

impl<K, V> SharedCache<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone,
    V: Clone,
{
    fn new() -> Rc<Self> {
        Rc::new(SharedCache {
            data: RefCell::new(HashMap::new()),
        })
    }

    fn get(&self, key: &K) -> Option<V> {
        self.data.borrow().get(key).cloned()
    }

    fn insert(&self, key: K, value: V) {
        self.data.borrow_mut().insert(key, value);
    }

    fn len(&self) -> usize {
        self.data.borrow().len()
    }

    fn is_empty(&self) -> bool {
        self.data.borrow().is_empty()
    }
}

// Exercise 4 Solution:
struct Component {
    name: String,
    game_object: Weak<GameObject>,
}

struct GameObject {
    name: String,
    components: RefCell<Vec<Rc<Component>>>,
}

impl Component {
    fn new(name: &str, game_object: &Rc<GameObject>) -> Rc<Self> {
        Rc::new(Component {
            name: name.to_string(),
            game_object: Rc::downgrade(game_object),
        })
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn game_object(&self) -> Option<Rc<GameObject>> {
        self.game_object.upgrade()
    }
}

impl GameObject {
    fn new(name: &str) -> Rc<Self> {
        Rc::new(GameObject {
            name: name.to_string(),
            components: RefCell::new(Vec::new()),
        })
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn add_component(&self, component: Rc<Component>) {
        self.components.borrow_mut().push(component);
    }

    fn get_component(&self, name: &str) -> Option<Rc<Component>> {
        self.components.borrow().iter()
            .find(|c| c.name() == name)
            .cloned()
    }

    fn get_components(&self) -> Vec<Rc<Component>> {
        self.components.borrow().clone()
    }
}

// Exercise 5 Solution:
struct MemoryPool<T> {
    data: Vec<Option<T>>,
    free_indices: Vec<usize>,
    allocated: usize,
}

impl<T> MemoryPool<T> {
    fn new(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            data.push(None);
        }

        let mut free_indices = Vec::with_capacity(capacity);
        for i in 0..capacity {
            free_indices.push(i);
        }

        MemoryPool {
            data,
            free_indices,
            allocated: 0,
        }
    }

    fn allocate(&mut self, value: T) -> Option<usize> {
        if let Some(index) = self.free_indices.pop() {
            self.data[index] = Some(value);
            self.allocated += 1;
            Some(index)
        } else {
            None
        }
    }

    fn deallocate(&mut self, index: usize) -> bool {
        if index >= self.data.len() || self.data[index].is_none() {
            return false;
        }

        self.data[index] = None;
        self.free_indices.push(index);
        self.allocated -= 1;
        true
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.data.len() {
            return None;
        }

        match &self.data[index] {
            Some(value) => Some(value),
            None => None,
        }
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.data.len() {
            return None;
        }

        match &mut self.data[index] {
            Some(value) => Some(value),
            None => None,
        }
    }

    fn allocated_count(&self) -> usize {
        self.allocated
    }
}

impl<T> Drop for MemoryPool<T> {
    fn drop(&mut self) {
        println!("Dropping MemoryPool with {} allocated objects", self.allocated);
    }
}
*/
