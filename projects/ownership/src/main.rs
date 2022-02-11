fn main() {
    println!("Hello, ownership!");
    // string literal, inmutable
    let s = "hello";
    // String allocated in the heap.
    let mut the_string = String::from("hello");
    the_string.push_str(", world");
    println!("the_string value: {}", the_string);
    // Passing ownership
    let s1 = String::from("hello");
    let s2 = s1;
    // don't work, s1 is invalid
    //println!("Is s1 alive? :{}", s1);
    // CLONE instead
    let s3 = s2.clone();
    println!("Is s2 alive? :{}", s2);
    println!("Is s3 a clone of s2? :{}", s3);
    //
    //
    // The same for functions
    // Type with heap memory has ownership
    // scalar who cares :-)
    let x = 5;
    get_scalar(x);
    println!("Scalar still alive :{}", x);
    //
    let s4 = String::from("hello");
    takes_ownership(s4);
    // Error of 'borrow of moved value'
    // println!("Try to use after left ownership:{}", s4);
    //
    // but ...now goes and back.
    let s5 = String::from("hello");
    println!("and back ... {}", takes_and_gives_back(s5));
    //
    //
    //
    // BOOM ... and the references were created.
    let mut s6 = String::from("hello s6");
    println!("borrows ... {}", calculate_length(&s6));
    println!("mutable reference ... {}", calculate_length_mut(&mut s6));
}

fn get_scalar(a: u32) {
    println!("Scalar parameter:{}", a);
}
fn takes_ownership(s: String) {
    println!("Complex-heap memory parameter:{}", s);
}
fn takes_and_gives_back(s: String) -> String {
    println!("Complex-heap memory parameter:{}", s);
    s
}
fn calculate_length(s: &String) -> usize {
    // It's borrow cannot change it.
    //s.push_str("add extra");
    s.len()
}
fn calculate_length_mut(s: &mut String) -> usize {
    // It's mutable !!!
    s.push_str("add extra");
    s.len()
}
