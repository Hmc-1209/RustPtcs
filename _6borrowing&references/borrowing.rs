fn main() {
  let mut x: i32 = 5;
  let y: &mut i32 = &mut x;
  *y += 1;
  
  // println!("{}", x); <-- Cannot read if already borrow to others
  println!("{}", y);
}