use std::{sync::{Arc, Mutex}, thread, time::Duration};

fn main() {
    // Shared mutable data across threads
    let shared_data = Arc::new(Mutex::new("Ashwin".to_string()));
    
    // Vector to handle threads
    let mut handles = vec![];
    
    // Create threads and mutate shared data
    for i in 1..4 {  
        let mutex_data = Arc::clone(&shared_data);
        
        let handle = thread::spawn(move || {
            // Acquire lock for exclusive access
            let mut data_in_thread = mutex_data.lock().unwrap();
            
            // Mutate the data
            data_in_thread.push_str(&format!(" (thread {})", i));
            
            // Log the mutated data ; just a dereference
            println!("Thread {}: {}", i, *data_in_thread);
            
            // Simulate some work
            thread::sleep(Duration::from_millis(100));
            
            // data_in_thread (MutexGuard) gets dropped here, releasing the lock
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Check final result
    match shared_data.lock() {
        Ok(data) => {
            println!("Final data: {}", *data);
        }
        Err(poisoned) => {
            println!("Mutex was poisoned, but data: {}", *poisoned.into_inner());
        }
    }
}

// Simple single-threaded Mutex
fn single_threaded_mutex_example() {
    println!("=== SINGLE-THREADED MUTEX EXAMPLE ===");
    
    let data = Mutex::new(vec![1, 2, 3]);
    println!("Initial: {:?}", data.lock().unwrap());
    
    // Modify data
    {
        let mut locked_data = data.lock().unwrap();
        locked_data.push(4);
        locked_data.push(5);
        println!("Modified: {:?}", *locked_data);
        // Lock released when locked_data goes out of scope
    }
    
    println!("Final: {:?}", data.lock().unwrap());
}