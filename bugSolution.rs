fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    // Correct way: use a reference to the vector instead of a slice.
    let vec_ref = &vec;
    println!("The vector is {:?}", vec_ref);
}