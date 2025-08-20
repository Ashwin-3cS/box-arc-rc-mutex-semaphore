use std::{cell::RefCell, rc::{self, Rc}};



fn main () {
//initialise the vector
    let var_vec = vec![1,2,3,4];


    //we are gonna rc it
    // in rc'ing the data like wrapping it with rc means you create co-owners ; not a single owner
    // this is the co-owner even though multiple people can rc.clone() it 
    let rc_var_vec = Rc::new(var_vec);
    println!("rc'ed vec {:?}",rc_var_vec);

    // logging the pointer now 
    let ptr = rc_var_vec.as_ptr();
    println!( "this the rc'ed vec ptr {:?}",ptr);

    //log the counter 
    let counter_cur = Rc::strong_count(&rc_var_vec);
    println!("{:?} this is the first data rc_var_vec",counter_cur);

    // now another 
    let rc_var_vec2 =  Rc::clone(&rc_var_vec);
    
    // logging the pointer now 
    let ptr = rc_var_vec2.as_ptr();
    println!( "this the rc'ed vec2 ptr {:?}",ptr);

    // now another
    let rc_var_vec3 = Rc::clone(&rc_var_vec2);
    println!("{:?} just logging the data",rc_var_vec3);
    println!("{:?} this is the counter of rC_vec3",Rc::strong_count(&rc_var_vec3));  // every one of these var's have equal ownership on the data

    // drop(rc_var_vec);


    
    // let me try to mutate but that's the problem //RC provides immutable access only 
    // then we can use RefCell<T>



    // notice the ownership as well (remains the same) // when we use the RefCell we can tap into .borrow() and .borrow_mut()
    // let me create a new var
    let mutable_vec = vec![1,2,3,4];
    let rc_mutable_vec = Rc::new(RefCell::new(mutable_vec));



    let borrowed_data_from_rc_mut_vec = rc_mutable_vec.borrow();
    println!("{:?}",borrowed_data_from_rc_mut_vec);


    println!("{:?}",Rc::strong_count(&rc_mutable_vec));

     // as we know only one var can mutate you know
    let mut_borrow_data_from_rc_mut_vec = rc_mutable_vec.borrow_mut().extend([1,2]);


    println!("{:?} this is the array after mut", mut_borrow_data_from_rc_mut_vec);







    


}