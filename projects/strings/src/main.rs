fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("strings!");
    let s3 = s1 + &s2;
    println!("{}", s3);
    // Cannot index String as array
    //let h = s3[2];

    // Loop CHARS
    for c in s3.chars() {
        println!("Characters of String {}", c);
    }

    // and Loop BYTES
    for b in s3.bytes() {
        println!("Bytes of String {}", b);
    }
}
