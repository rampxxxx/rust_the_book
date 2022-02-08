fn main() {
    println!("Hello, world!");
    another_function(5);
    println!("value from fn five {}", five());
    //println!("value from fn six {}", six());
}
fn another_function(p: u8) {
    println!("in function, value {}!", p);

    // This is not C
    // the '=" don't return the assigned value
    // a statement ... don't return a value
    let x = 1;
    // a expresion ... return a value.
    x + 1;

    // Curly braces power
    let y = {
        let x = 1;
        x + 1;
    };
    println!("the return of operation in curly braces {:?}!", y);
    // 'y' is EMPTY !!! ... is '()' , the UNIT TYPE
    // Curly braces power
    let y = {
        let x = 1;
        x + 1
    };
    println!("the return of operation in curly braces {:}!", y);

    //  x+1';' makes curly braces an statenent ... which don't return a value.
    //
    //  what, what, what
}

// Expression. Function with a 'return' value
fn five() -> i32 {
    5
}
// Statement , Function with a 'return' value . DON'T compile, make fn Unit Type.
//fn six() -> i32 {
//    6;
//}
