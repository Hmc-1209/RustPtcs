fn main() {
    let mut s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    s1 = s1 + &s2;
    println!("{}", s1);

    let s3: String = String::from("Hi ");
    let s4: String = String::from("there!");
    println!("{}", format!("{}{}", s3, s4));
}