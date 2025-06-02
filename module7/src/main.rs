// Module 7: Concurrency and Parallelism
// This module explores Rust's approach to concurrent and parallel programming:
// - Thread safety through the type system
// - Send and Sync traits
// - Mutex and Arc internals
// - Channel types and message passing
// - Async/await fundamentals
// - Future trait and executors
// - Tokio runtime deep dive
// - Common concurrency patterns

mod problems;

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    // Run the practice problems
    problems::run_exercises();

    // ===============================
    // 1. Thread Safety in Rust
    // ===============================
    println!("\n1. Thread Safety in Rust:");
    println!("----------------------");
    // Rust's type system enforces thread safety at compile time
    // - Send: Types that can be transferred between threads
    // - Sync: Types that can be shared between threads

    // Creating threads
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
        for i in 1..5 {
            println!("Thread: count {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    // Main thread continues execution
    for i in 1..5 {
        println!("Main: count {}", i);
        thread::sleep(Duration::from_millis(10));
    }

    // Wait for the spawned thread to finish
    handle.join().unwrap();

    // ===============================
    // 2. Send and Sync Traits
    // ===============================
    println!("\n2. Send and Sync Traits:");
    println!("---------------------");
    // Send: Types that can be transferred between threads
    // Sync: Types that can be shared between threads (&T is Send)

    // Examples of Send and !Send types
    println!("Examples of Send types:");
    println!("  - i32, String, Vec<T> where T: Send");
    println!("  - Arc<T> where T: Send + Sync");
    println!("Examples of !Send types:");
    println!("  - Rc<T> (not thread-safe reference counting)");
    println!("  - RefCell<T> (not thread-safe interior mutability)");

    // Examples of Sync and !Sync types
    println!("\nExamples of Sync types:");
    println!("  - i32, String, Vec<T> where T: Sync");
    println!("  - Mutex<T> where T: Send");
    println!("Examples of !Sync types:");
    println!("  - Cell<T>, RefCell<T> (interior mutability)");
    println!("  - *mut T (raw pointers)");

    // ===============================
    // 3. Mutex and Arc Internals
    // ===============================
    println!("\n3. Mutex and Arc Internals:");
    println!("------------------------");
    // Mutex: Mutual exclusion with RAII guards
    // Arc: Atomic reference counting for thread-safe sharing

    // Creating a shared counter
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            // Lock is automatically released when num goes out of scope
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());

    // Mutex internals
    println!("\nMutex internals:");
    println!("  - Contains the data and a platform-specific lock");
    println!("  - lock() returns a MutexGuard (RAII pattern)");
    println!("  - MutexGuard implements Deref and Drop");
    println!("  - Lock is released when MutexGuard is dropped");

    // Arc internals
    println!("\nArc internals:");
    println!("  - Contains a pointer to heap-allocated data");
    println!("  - Strong and weak counts are atomic integers");
    println!("  - clone() increments the strong count atomically");
    println!("  - drop() decrements the strong count atomically");
    println!("  - Data is freed when strong count reaches zero");

    // ===============================
    // 4. RwLock and Other Synchronization Primitives
    // ===============================
    println!("\n4. RwLock and Other Synchronization Primitives:");
    println!("------------------------------------------");
    // RwLock: Multiple readers or single writer
    // Barrier: Synchronization point for multiple threads
    // Condvar: Condition variables for signaling

    // RwLock example
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    // Spawn multiple reader threads
    let mut reader_handles = vec![];
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let data = data.read().unwrap();
            println!("Reader {}: {:?}", i, *data);
        });
        reader_handles.push(handle);
    }

    // Spawn a writer thread
    let data_clone = Arc::clone(&data);
    let writer_handle = thread::spawn(move || {
        let mut data = data_clone.write().unwrap();
        data.push(4);
        println!("Writer: {:?}", *data);
    });

    // Wait for all threads
    for handle in reader_handles {
        handle.join().unwrap();
    }
    writer_handle.join().unwrap();

    // ===============================
    // 5. Channel Types and Message Passing
    // ===============================
    println!("\n5. Channel Types and Message Passing:");
    println!("----------------------------------");
    // Channels provide a way to send data between threads
    // - mpsc: Multi-producer, single-consumer
    // - crossbeam: Multi-producer, multi-consumer

    // Basic channel example
    let (tx, rx) = std::sync::mpsc::channel();

    thread::spawn(move || {
        tx.send("Hello from thread").unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Received: {}", received);

    // Multiple producers
    let (tx, rx) = std::sync::mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        tx1.send("Message from thread 1").unwrap();
    });

    thread::spawn(move || {
        tx.send("Message from thread 2").unwrap();
    });

    for _ in 0..2 {
        println!("Received: {}", rx.recv().unwrap());
    }

    // ===============================
    // 6. Async/Await Fundamentals
    // ===============================
    println!("\n6. Async/Await Fundamentals:");
    println!("-------------------------");
    // Async/await provides a way to write asynchronous code
    // that looks like synchronous code
    // - async fn returns a Future
    // - await suspends execution until Future completes
    // - Requires an executor to run Futures

    println!("Async function example:");
    println!("  async fn fetch_data() -> Result<String, Error> {{");
    println!("      let response = client.get(url).await?;");
    println!("      let body = response.text().await?;");
    println!("      Ok(body)");
    println!("  }}");

    println!("\nRunning async code:");
    println!("  #[tokio::main]");
    println!("  async fn main() {{");
    println!("      let result = fetch_data().await;");
    println!("      println!(\"Result: {{}}\", result);");
    println!("  }}");

    // ===============================
    // 7. Future Trait and Executors
    // ===============================
    println!("\n7. Future Trait and Executors:");
    println!("---------------------------");
    // Future trait is the core of async/await
    // - poll() method returns Poll::Ready or Poll::Pending
    // - Executors manage and run Futures
    // - Wakers notify executors when a Future is ready

    println!("Future trait:");
    println!("  trait Future {{");
    println!("      type Output;");
    println!("      fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;");
    println!("  }}");

    println!("\nExecutor responsibilities:");
    println!("  - Scheduling Futures");
    println!("  - Polling Futures when they're ready");
    println!("  - Managing wakeups via Waker");
    println!("  - Handling I/O and timers");

    // ===============================
    // 8. Tokio Runtime Deep Dive
    // ===============================
    println!("\n8. Tokio Runtime Deep Dive:");
    println!("------------------------");
    // Tokio is a popular async runtime for Rust
    // - Multi-threaded runtime
    // - Work-stealing scheduler
    // - Efficient I/O and timer subsystems

    println!("Tokio components:");
    println!("  - Runtime: Manages the executor and resources");
    println!("  - Scheduler: Distributes tasks across worker threads");
    println!("  - I/O Driver: Non-blocking I/O operations");
    println!("  - Timer Wheel: Efficient timing operations");

    println!("\nTokio usage example:");
    println!("  #[tokio::main]");
    println!("  async fn main() -> Result<(), Error> {{");
    println!("      let mut listener = TcpListener::bind(\"127.0.0.1:8080\").await?;");
    println!("      loop {{");
    println!("          let (socket, _) = listener.accept().await?;");
    println!("          tokio::spawn(async move {{");
    println!("              process_socket(socket).await;");
    println!("          }});");
    println!("      }}");
    println!("  }}");
}

// Notes on Concurrency and Parallelism:
//
// 1. Thread Safety
//    - Rust's type system prevents data races at compile time
//    - Send and Sync traits enforce thread safety
//    - No shared mutable state without synchronization
//    - No null pointers or dangling references
//
// 2. Synchronization Primitives
//    - Mutex<T>: Mutual exclusion with RAII guards
//    - RwLock<T>: Multiple readers or single writer
//    - Arc<T>: Thread-safe reference counting
//    - Atomic types: Lock-free operations
//    - Channels: Message passing between threads
//
// 3. Async/Await
//    - Zero-cost abstraction over callbacks
//    - Future trait represents a value that will be available later
//    - Executors manage and run Futures
//    - Tokio, async-std, and smol are popular runtimes
//
// 4. Performance Characteristics
//    - Threads: OS-level scheduling, higher overhead
//    - Async: User-level scheduling, lower overhead
//    - Atomics: Hardware-level synchronization
//    - Channels: Message passing with queue semantics
//
// 5. Best Practices
//    - Prefer message passing over shared state
//    - Use the right tool for the job (threads vs async)
//    - Minimize critical sections
//    - Avoid blocking in async code
//    - Consider work-stealing for CPU-bound tasks

// Try experimenting with these concepts:
// 1. Create a thread pool for parallel processing
// 2. Implement a producer-consumer pattern with channels
// 3. Use async/await with Tokio for concurrent I/O
// 4. Explore atomic operations for lock-free algorithms
// 5. Implement a custom Future type
