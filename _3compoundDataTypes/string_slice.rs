fn main() {
  let string: String = String::from("Hello, World!");
  let slice: &str = &string;
  let slice2: &str = &string[0..5];
  println!("{}", slice);
  println!("{}", slice2)
}