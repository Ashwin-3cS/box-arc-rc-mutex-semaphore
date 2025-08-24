use std::{sync::{Arc, Mutex}, time::Duration};
use tokio::sync::Semaphore;

async fn semaphore_fun() {
    // Initialize semaphore - thread safe so we wrap it with Arc instead of Rc
    let semaphore = Arc::new(Semaphore::new(2)); // Set semaphore to 2 permits
    
    // Initialize shared state that will be mutated across threads
    let shared_counter_mut = Arc::new(Mutex::new(0));
    
    // Create thread handle vector to store multiple thread JoinHandles
    let mut handles = vec![];
    
    for i in 1..=4 {
        // Clone the semaphore so each thread gets its own reference
        let sem = Arc::clone(&semaphore);
        
        // Clone the shared counter so each thread can access it
        let shared_counter_mut_data_cloned = Arc::clone(&shared_counter_mut);
        
        let handle = tokio::spawn(async move {
            println!("Thread {} waiting for permit...", i);
            
            // Acquire permit - only 2 threads can get this at a time
            let _acq_sem = sem.acquire().await.unwrap();
            
            println!("Thread {} got permit! Available permits: {}", i, sem.available_permits());
            
            // Do some work while holding the permit and also update shared counter
            {
                let mut counter = shared_counter_mut_data_cloned.lock().unwrap();
                *counter += 1;
                println!("Thread {} updated counter to: {}", i, *counter);
            }
            
            // Simulate work for 500ms while holding the permit
            tokio::time::sleep(Duration::from_millis(500)).await;
            
            println!("Thread {} releasing permit", i);
            // Permit automatically released when _acq_sem drops
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.await.unwrap();
    }
    
    println!("All threads completed!");
    println!("Final counter value: {}", *shared_counter_mut.lock().unwrap());
}


#[tokio::main]
async fn main() {
    semaphore_fun().await;
    
}