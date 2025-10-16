fn main() {
  let numbers: &[i32] = &[1, 2, 3, 4, 5];
  println!("Number slices: {:?}", numbers);

  let animals: &[&str] = &["Dog", "Cat", "Mouse"];
  println!("Animal slices: {:?}", animals);

  let books: &[&String] = &[&"IT".to_string(), &"Phy".to_string(), &"Geo".to_string()];
  println!("Animal slices: {:?}", books);
}