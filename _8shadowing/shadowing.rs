fn main() {
  let x: i32 = 5;
  let x: i32 = x + 1;
  println!("x is {}", x);

  {
    let x: i32 = x * 2;
    println!("x is {}", x);
  }
}