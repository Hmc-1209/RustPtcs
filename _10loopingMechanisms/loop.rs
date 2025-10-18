fn main() {
  let mut x: i32 = 10;

  let result: i32 = loop {
    x += 1;
    if x >= 20 {
      break x - 10;
    }
  };

  println!("Result: {}", result);
}