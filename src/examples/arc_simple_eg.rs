use std::{sync::Arc, thread, time::Duration};



fn main () {


    let sampl_data = vec![1,12];

    // we use arc when we need to have data moved between threads since RC is not threadSafe
    let arc_samp_data = Arc::new(sampl_data);
    println!("{:?}this is the counter owner1",Arc::strong_count(&arc_samp_data));

    let mut handles = vec![];

    

    for _i in 1..4 {

        let arc_cloned_var = Arc::clone(&arc_samp_data);
        let handle  =  thread::spawn(move || {
            println!("{:?} just some task with this data",*arc_cloned_var); //its always good to dereference manually insted of relying on macro's 
            


            thread::sleep(Duration::from_secs(1));


            println! ("{:?} , this is the thread currently thats taking up this task",thread::current().id())
        });

        handles.push(handle);
    }

    for handle in handles { 
        handle.join().unwrap();
    }
    

}