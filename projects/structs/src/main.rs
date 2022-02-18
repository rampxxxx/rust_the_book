fn main() {
    println!("Hello, world!");
    //#[attribute(Trait)]
    #[derive(Debug)]
    struct St1 {
        name: String,
        count: u32,
    }

    impl St1 {
        fn new(name: String, count: u32) -> Self {
            Self { name, count }
        }
        fn get(&self) -> &Self {
            self
        }
    }

    let s1 = St1 {
        name: "javier".to_string(),
        count: 1,
    };
    let s2 = St1 {
        name: "luis".to_string(),
        ..s1
    };

    //let s3 = St1 { ..s1 }; // Borrow happens.
    //
    println!("Struct value {}-{}\n", s1.name, s1.count);
    // With debugging support
    println!("[derive(Debug)] Struct value {:?}\n", s1);
    //
    // unit struct
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    // Cannot print :-( unit struct.
    //println!("Print unit struct {}\n", subject);
    //
    //
    // Printing to stderr
    dbg!(&s1); // &s1, because takes ownership
    println!("[derive(Debug)] Struct value {:?}\n", s1);
    //
    //
    let name = String::from("Carlos");
    let s4 = St1::new(name, 2);
    println!("Struct St1 method get {:?}\n", s4.get());
    println!("Struct St1 method get {:?}\n", s4.get());
}
