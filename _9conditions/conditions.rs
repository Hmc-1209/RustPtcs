fn main() {
  let weather: String = String::from("Rainy");

  if weather == "Rainy".to_string() {
    println!("Remember to bring the unbrella! It is raining outside.");
  } else if weather == "Sunny".to_string() {
    println!("Remember to bring the unbrella! The sun is big.");
  } else {
    println!("You don't need the umbrella now!");
  }

  let bring_umbrella: bool = if weather == "Rainy" || weather == "Sunny" {true} else {false};
  println!("Bring umbrella: {}", bring_umbrella);
}