fn main() {
    let mut v1: Vec<i32> = Vec::new();  // Defining with empty vector
    let mut v2: Vec<i32> = vec![1, 2, 3];   // Defining with value

    v1.push(1);

    println!("The vector v1: {:?}.", v1);
    println!("The third element in v2: {}.", &v2[2]);

    let forth = v2.get(4);
    match forth {
        Some(x) => println!("The third element in v2 is {}.", x),
        None => println!("There is no forth element in v2."),
    }
}