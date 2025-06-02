// Module 7: Practice Problems
// These exercises focus on concurrency and parallelism in Rust.
// Each problem includes detailed explanations of the underlying concepts
// and how Rust's type system ensures thread safety.

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

pub fn run_exercises() {
    println!("Module 7 Exercises - Concurrency and Parallelism");
    println!("==========================================\n");

    exercise1();
    exercise2();
    exercise3();
    exercise4();
    exercise5();
}

// Exercise 1: Thread Basics
// This exercise demonstrates:
// - Creating and joining threads
// - Sharing data between threads
// - Thread safety with Arc and Mutex
fn exercise1() {
    println!("Exercise 1: Thread Basics");
    println!("---------------------");
    println!("TODO: Implement the parallel_sum function\n");

    // Function to sum a vector in parallel
    fn parallel_sum(data: &[i32], num_threads: usize) -> i32 {
        unimplemented!("Implement parallel_sum");
    }

    // Test your implementation:
    // let data: Vec<i32> = (1..=1000).collect();
    // let sum = parallel_sum(&data, 4);
    // println!("Sum: {}", sum);
    // assert_eq!(sum, data.iter().sum());
}

// Exercise 2: Producer-Consumer Pattern
// This exercise demonstrates:
// - Channel-based communication
// - Multiple producer, single consumer pattern
// - Coordinating work between threads
fn exercise2() {
    println!("\nExercise 2: Producer-Consumer Pattern");
    println!("--------------------------------");
    println!("TODO: Implement the WorkQueue struct and its methods\n");

    // A work queue that distributes tasks to worker threads
    struct WorkQueue<T, R> {
        // TODO: Implement the fields needed for a work queue
    }

    impl<T: Send + 'static, R: Send + 'static> WorkQueue<T, R> {
        // Create a new work queue with the specified number of workers
        fn new<F>(num_workers: usize, handler: F) -> Self
        where
            F: Fn(T) -> R + Send + Sync + 'static,
        {
            unimplemented!("Implement WorkQueue::new");
        }

        // Add a task to the queue
        fn add_task(&self, task: T) {
            unimplemented!("Implement add_task");
        }

        // Get a result from the queue
        fn get_result(&self) -> Option<R> {
            unimplemented!("Implement get_result");
        }
    }

    // Test your implementation:
    // let queue = WorkQueue::new(4, |n: i32| {
    //     // Simulate work
    //     thread::sleep(Duration::from_millis(10));
    //     n * n
    // });
    //
    // // Add tasks
    // for i in 1..=10 {
    //     queue.add_task(i);
    // }
    //
    // // Get results
    // let mut results = Vec::new();
    // for _ in 1..=10 {
    //     if let Some(result) = queue.get_result() {
    //         results.push(result);
    //     }
    // }
    //
    // println!("Results: {:?}", results);
    // assert_eq!(results.len(), 10);
}

// Exercise 3: Reader-Writer Lock
// This exercise demonstrates:
// - Multiple readers, single writer pattern
// - RwLock usage and internals
// - Preventing data races
fn exercise3() {
    println!("\nExercise 3: Reader-Writer Lock");
    println!("--------------------------");
    println!("TODO: Implement the ConcurrentCache struct and its methods\n");

    // A concurrent cache that allows multiple readers but only one writer
    struct ConcurrentCache<K, V> {
        // TODO: Implement the fields needed for a concurrent cache
    }

    impl<K, V> ConcurrentCache<K, V>
    where
        K: std::cmp::Eq + std::hash::Hash + Clone + Send + Sync + 'static,
        V: Clone + Send + Sync + 'static,
    {
        // Create a new concurrent cache
        fn new() -> Self {
            unimplemented!("Implement ConcurrentCache::new");
        }

        // Get a value from the cache
        fn get(&self, key: &K) -> Option<V> {
            unimplemented!("Implement get");
        }

        // Insert a value into the cache
        fn insert(&self, key: K, value: V) {
            unimplemented!("Implement insert");
        }

        // Remove a value from the cache
        fn remove(&self, key: &K) -> bool {
            unimplemented!("Implement remove");
        }
    }

    // Test your implementation:
    // let cache = ConcurrentCache::new();
    // let cache_arc = Arc::new(cache);
    //
    // // Spawn reader threads
    // let mut handles = vec![];
    // for i in 0..5 {
    //     let cache = Arc::clone(&cache_arc);
    //     let handle = thread::spawn(move || {
    //         for j in 0..100 {
    //             if let Some(value) = cache.get(&j) {
    //                 println!("Reader {}: key={}, value={}", i, j, value);
    //             }
    //             thread::sleep(Duration::from_millis(1));
    //         }
    //     });
    //     handles.push(handle);
    // }
    //
    // // Spawn writer threads
    // for i in 0..3 {
    //     let cache = Arc::clone(&cache_arc);
    //     let handle = thread::spawn(move || {
    //         for j in 0..100 {
    //             cache.insert(j, format!("value-{}-{}", i, j));
    //             thread::sleep(Duration::from_millis(5));
    //         }
    //     });
    //     handles.push(handle);
    // }
    //
    // // Wait for all threads to finish
    // for handle in handles {
    //     handle.join().unwrap();
    // }
}

// Exercise 4: Atomic Operations
// This exercise demonstrates:
// - Lock-free programming
// - Atomic types and operations
// - Memory ordering
fn exercise4() {
    println!("\nExercise 4: Atomic Operations");
    println!("-------------------------");
    println!("TODO: Implement the AtomicCounter struct and its methods\n");

    use std::sync::atomic::{AtomicUsize, Ordering};

    // A thread-safe counter using atomic operations
    struct AtomicCounter {
        // TODO: Implement the fields needed for an atomic counter
    }

    impl AtomicCounter {
        // Create a new atomic counter
        fn new() -> Self {
            unimplemented!("Implement AtomicCounter::new");
        }

        // Increment the counter and return the previous value
        fn increment(&self) -> usize {
            unimplemented!("Implement increment");
        }

        // Decrement the counter and return the previous value
        fn decrement(&self) -> usize {
            unimplemented!("Implement decrement");
        }

        // Get the current value
        fn get(&self) -> usize {
            unimplemented!("Implement get");
        }
    }

    // Test your implementation:
    // let counter = Arc::new(AtomicCounter::new());
    // let mut handles = vec![];
    //
    // // Spawn incrementer threads
    // for _ in 0..5 {
    //     let counter = Arc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         for _ in 0..1000 {
    //             counter.increment();
    //         }
    //     });
    //     handles.push(handle);
    // }
    //
    // // Spawn decrementer threads
    // for _ in 0..5 {
    //     let counter = Arc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         for _ in 0..500 {
    //             counter.decrement();
    //         }
    //     });
    //     handles.push(handle);
    // }
    //
    // // Wait for all threads to finish
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    //
    // println!("Final counter value: {}", counter.get());
    // assert_eq!(counter.get(), 2500); // 5*1000 - 5*500 = 2500
}

// Exercise 5: Async Programming
// This exercise demonstrates:
// - Async/await syntax
// - Future trait
// - Custom executor
fn exercise5() {
    println!("\nExercise 5: Async Programming");
    println!("-------------------------");
    println!("TODO: Implement the mini_executor function\n");

    // Note: This is a simplified version of async concepts
    // In a real application, you would use a library like tokio or async-std

    // A simple future that completes after a delay
    struct Delay {
        duration: Duration,
    }

    impl Delay {
        fn new(duration: Duration) -> Self {
            Delay { duration }
        }
    }

    // A simple executor that runs futures to completion
    fn mini_executor<F>(future: F) -> F::Output
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        unimplemented!("Implement mini_executor");
    }

    // Test your implementation:
    // Note: This would require the full async/await machinery to work
    // In a real implementation, you would use:
    //
    // async fn async_task() -> i32 {
    //     let delay = Delay::new(Duration::from_millis(100));
    //     delay.await;
    //     42
    // }
    //
    // let result = mini_executor(async_task());
    // println!("Result: {}", result);
}

/* Example Solutions (Try to solve the exercises before looking at these!)

// Exercise 1 Solution:
fn parallel_sum(data: &[i32], num_threads: usize) -> i32 {
    if data.is_empty() {
        return 0;
    }

    // Handle edge case where we have more threads than data
    let num_threads = std::cmp::min(num_threads, data.len());

    // Calculate chunk size for each thread
    let chunk_size = (data.len() + num_threads - 1) / num_threads;

    // Create a vector to store thread handles
    let mut handles = Vec::with_capacity(num_threads);

    // Spawn threads to process chunks
    for i in 0..num_threads {
        let start = i * chunk_size;
        let end = std::cmp::min(start + chunk_size, data.len());

        // Clone the slice for this thread
        let chunk = data[start..end].to_vec();

        // Spawn thread to sum this chunk
        let handle = thread::spawn(move || {
            chunk.iter().sum::<i32>()
        });

        handles.push(handle);
    }

    // Collect results from all threads
    let mut total = 0;
    for handle in handles {
        total += handle.join().unwrap();
    }

    total
}

// Exercise 2 Solution:
struct WorkQueue<T, R> {
    task_sender: std::sync::mpsc::Sender<T>,
    result_receiver: std::sync::mpsc::Receiver<R>,
}

impl<T: Send + 'static, R: Send + 'static> WorkQueue<T, R> {
    fn new<F>(num_workers: usize, handler: F) -> Self
    where
        F: Fn(T) -> R + Send + Sync + 'static,
    {
        let (task_sender, task_receiver) = std::sync::mpsc::channel();
        let (result_sender, result_receiver) = std::sync::mpsc::channel();

        // Create a thread-safe reference to the task receiver
        let task_receiver = Arc::new(Mutex::new(task_receiver));

        // Create a thread-safe reference to the handler
        let handler = Arc::new(handler);

        // Spawn worker threads
        for _ in 0..num_workers {
            let task_receiver = Arc::clone(&task_receiver);
            let result_sender = result_sender.clone();
            let handler = Arc::clone(&handler);

            thread::spawn(move || {
                loop {
                    // Get a task from the queue
                    let task = {
                        let receiver = task_receiver.lock().unwrap();
                        match receiver.recv() {
                            Ok(task) => task,
                            Err(_) => break, // Channel closed, exit thread
                        }
                    };

                    // Process the task
                    let result = handler(task);

                    // Send the result
                    if result_sender.send(result).is_err() {
                        break; // Receiver dropped, exit thread
                    }
                }
            });
        }

        WorkQueue {
            task_sender,
            result_receiver,
        }
    }

    fn add_task(&self, task: T) {
        self.task_sender.send(task).unwrap();
    }

    fn get_result(&self) -> Option<R> {
        self.result_receiver.recv().ok()
    }
}

// Exercise 3 Solution:
use std::collections::HashMap;

struct ConcurrentCache<K, V> {
    cache: RwLock<HashMap<K, V>>,
}

impl<K, V> ConcurrentCache<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn new() -> Self {
        ConcurrentCache {
            cache: RwLock::new(HashMap::new()),
        }
    }

    fn get(&self, key: &K) -> Option<V> {
        // Acquire read lock
        let cache = self.cache.read().unwrap();
        cache.get(key).cloned()
    }

    fn insert(&self, key: K, value: V) {
        // Acquire write lock
        let mut cache = self.cache.write().unwrap();
        cache.insert(key, value);
    }

    fn remove(&self, key: &K) -> bool {
        // Acquire write lock
        let mut cache = self.cache.write().unwrap();
        cache.remove(key).is_some()
    }
}

// Exercise 4 Solution:
struct AtomicCounter {
    count: AtomicUsize,
}

impl AtomicCounter {
    fn new() -> Self {
        AtomicCounter {
            count: AtomicUsize::new(0),
        }
    }

    fn increment(&self) -> usize {
        self.count.fetch_add(1, Ordering::SeqCst)
    }

    fn decrement(&self) -> usize {
        self.count.fetch_sub(1, Ordering::SeqCst)
    }

    fn get(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }
}

// Exercise 5 Solution:
// Note: This is a simplified version that doesn't actually implement a real executor
// A real executor would be much more complex and use polling
fn mini_executor<F>(future: F) -> F::Output
where
    F: std::future::Future + Send + 'static,
    F::Output: Send + 'static,
{
    // Create a channel to receive the result
    let (sender, receiver) = std::sync::mpsc::channel();

    // Spawn a thread to run the future
    thread::spawn(move || {
        // In a real executor, we would poll the future until it's ready
        // For this simplified version, we'll just pretend we can get the output directly
        // This is not how real async works!

        // This is just a placeholder - in reality, you'd need a real executor
        let output = unimplemented!("This requires a real async runtime");

        sender.send(output).unwrap();
    });

    // Wait for the result
    receiver.recv().unwrap()
}

// A more realistic (but still simplified) executor might look like:
// (Note: This is still not a complete implementation)
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Wake};
use std::sync::{Arc, Mutex};

struct SimpleExecutor;

impl SimpleExecutor {
    fn block_on<F: Future>(future: F) -> F::Output {
        // Create a waker that does nothing
        struct NoopWaker;
        impl Wake for NoopWaker {
            fn wake(self: Arc<Self>) {}
        }
        let waker = Arc::new(NoopWaker).into();
        let mut context = Context::from_waker(&waker);

        // Pin the future to the stack
        let mut future = Box::pin(future);

        // Poll the future until it's ready
        loop {
            match future.as_mut().poll(&mut context) {
                Poll::Ready(output) => return output,
                Poll::Pending => thread::yield_now(),
            }
        }
    }
}
*/
