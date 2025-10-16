fn main() {
  let nums: [i32; 5] = [1, 2, 3, 4, 5];
  println!("Numbers: {:?}", nums);

  let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
  println!("Fruits: {:?}", fruits);
  println!("First ruit: {}", fruits[0]);
  println!("Second ruit: {}", fruits[1]);
  println!("Third ruit: {}", fruits[2]);
}