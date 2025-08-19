#[derive(Debug)]
struct BigData { 
    data : [u8;1024]  // this means u8-> 1bytes and 1024 is the number of elements gonna be in this fixed array so 
    //1bytes*1024 -> 1024 values like 1's
    // data : [1,1,1,1,1,1,1,1, .......]
}


fn main () {
    let var1 = BigData {
        data : [1;1024]
    };

    let var2 = var1;
     // length of the data moved to the ownership is transferred to var 2 ?
     println!("this is the var2 data stack data entirely moved to var2 {:?}",var2);



     // now i box the data in 
     let var3 = Box::new(var2);
     println!("this is the var2 data stack data entirely moved to var3 {:p}",var3); // like the var3 holds the pointer only the data is actually in heap and metadaata as well
     // :p prints the address of the pointer in hexadecimal format because of debug trait 
     
     
}