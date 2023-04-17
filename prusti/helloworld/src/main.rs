fn main() {
    // Specify this condition to avoid unsupported features of Prusti.
    #[cfg(not(prusti))]
    println!("Hello, world!");
}
