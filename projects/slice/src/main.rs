/// Slice examples.
///
/// # Examples
///
/// ```
/// A slice is a king of reference, so it does not have ownership.
/// ```
fn main() {
    println!("Hello, slice");
    // This is already a slice '&str'
    let slice = "hello slice";
    //--------------------012345678901
    let s = String::from("hello, slice");
    let h = &s[0..6];
    let w = &s[7..12];
    println!("Hello, slice:{}", get_slice(1, 2, &s));
    //
    //Slice of int
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_ne!(slice, &[2, 3]);
}
fn get_slice(start: u32, end: u32, s: &str) -> &str {
    //Returning a slice type 'str'
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // space in string
            return &s[0..i];
        }
    }
    s
}
