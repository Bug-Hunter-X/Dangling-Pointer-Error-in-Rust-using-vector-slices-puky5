fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let slice = &vec[..];
    // This is incorrect.  It creates a dangling pointer.
    // The vec is dropped at the end of the function, so the memory that the slice points to will be invalid.
    println!("The slice is {:?}", slice);
}