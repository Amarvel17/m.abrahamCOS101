fn main(){
    // debugged code

    let v = vec![10,20,30];
    // vector v owns the object in heap

    let v2 = v.clone();  //moves ownership to v2

    display(v2);
    //v2 is moved to display and v2 is invalidated

    println!("In main {:?}",v);
}

fn display(v:Vec<i32>){
    println!("inside display{:?}",v );
}
