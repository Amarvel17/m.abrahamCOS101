fn main(){

    let v = vec![101, 250, 330, 400];
    // vector v owns the object in heap

    //only a single variable owns the heap memory ata any given time
    let _v2 = v;
    //here two variables own heap value,
    //two pointers to the same content is not allowed in rust

    //Rust is very smart in terms of memory access, so it detects a race condition
    //as two variables point to same heap
    // I have corrected the error in this code
    println!("{:?}",_v2);
}
