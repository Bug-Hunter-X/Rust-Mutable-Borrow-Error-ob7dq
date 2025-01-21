fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    *y += 1; // Modify x through y
    println!("x = {}", x); // x is now 6

    // To use z, we must drop y first
    let z = &x; // z is an immutable reference to x
    println!("z = {}", *z); // This is now valid
} 