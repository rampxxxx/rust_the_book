fn main() {
    let mut i = 0;
    loop {
        i += 1;
        if i > 10 {
            println!("Loop, ends!");
            break;
        } else {
            println!("Loop, {}!", i);
        }
    }

    // Returning value from loop
    let result = loop {
        i += 1;
        if i > 10 {
            println!("Loop, ends!");
            break i;
        } else {
            println!("Loop, {}!", i);
        }
    };
    println!("Returning Loop, {}!", result);

    while i > 0 {
        i -= 1;
        println!("while : cuenta atrás {}!", i);
    }

    // Modern loops.

    //
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("for : array elements {}!", element);
    }

    //
    for element in (1..4).rev() {
        println!("for : reverse (1..4) {}!", element);
    }
}
