pub fn double_free() {
    let a = String::from("hello"); //pointer to heap memory of hello created
    println!("{}", a); //valid here
    let b = a; //pointer got "moved" to b --> shallow copy + invalidating a

    //println!("{}", a); //this would be invalid herebecause a was moved to b
    println!("{}", b); //valid
}

pub fn dereferencing_freed_memory() {
    let a = Box::new(80i32);
    let b = a;

    take_ownership(b);

    //println!("{}", b); //results in a compile error as b got moved -> would be dereferencing freed memory
}

pub fn take_ownership(bx: Box<i32>){
    println!("Destorying the box which contains {}", bx);
}

pub fn borrowing() {
    let a = Box::new(80i32);
    let b = a;

    borrow_value(&b);

    println!("{}", b); //this now works :)
}

pub fn borrow_value(bx: &Box<i32>){
    println!("Borrowing the box which contains {}", bx);
}