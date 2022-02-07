const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
fn main() {
    let mut x = 5;
    println!("The value of x is : {}", x);
    x = 6;
    // Shadow vars
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x)
    }
    println!("The value of x in is: {}", x);
    //
    let spaces = "    ";
    let spaces = spaces.len();
    // tupple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Destructuring tupple by pattern matching
    let (x, y, z) = tup;
    println!(
        "Destructuring value of tuple elements x {} . y {} , z {}",
        x, y, z
    );
    println!(
        "Accesing elements of tuple x.0 {} . y.1 {} , z.2 {}",
        tup.0, tup.1, tup.2
    );
    // Unit type , Unit Value ....tupple
    let tup = ();
    println!("Accesing Unit type-value tupple {:?} ", tup);
    println!("Accesing Unit type-value tupple pretty {:#?} ", tup);
    // Arrays
    let a = [1, 2, 3, 4];
    let a: [i32; 4] = [1, 2, 3, 4];
    let a = [3; 5];
}
