fn main() {
    println!("Hello, collections!");

    //
    //let mut v: Vec<i32> = Vec::new();
    let mut v: Vec<i32> = vec![];
    v.push(2);
    v.push(2);
    // TWO ways of accessing.
    //println!("The third elements of the vector {}", v[2]); // PANIC , accessing not existing
    //item.
    // ALSO with get
    let one = v.get(0);
    println!("Vector one {:?}", Some(one));
    let two = &v[1];
    println!("Vector two {}", two); // Use before mut ref -push-
    v.push(2);
    match v.get(2) {
        Some(third_element) => println!("The third from match+get:{}", third_element),
        None => println!("This vector has no third element"),
    }

    { // In this case the use of inmut ref after -push- mut ref INVALID
         /*
                 let mut v = vec![1, 2, 3, 4, 5];

                 let first = &v[0];

                 v.push(6);

                 println!("The first element is: {}", first);
         */
    }

    // ITERATING : mut and !mut

    for i in &v {
        println!("Vector {}", i);
    }
    for i in &mut v {
        *i += 1;
        println!("Vector {}", i);
    }

    vector_with_different_types();
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
// Store different types in a vector.
fn vector_with_different_types() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text("hi spread sheet row".to_string()),
        SpreadsheetCell::Float(10.12),
    ];
    vector_show(&row);
}
// Store different types in a vector.
fn vector_show(v: &[SpreadsheetCell]) {
    for i in v {
        match i {
            SpreadsheetCell::Int(my_int) => println!("int {}", my_int),
            SpreadsheetCell::Float(my_float) => println!("float {}", my_float),
            SpreadsheetCell::Text(my_text) => println!("text {}", my_text),
        }
    }
}
