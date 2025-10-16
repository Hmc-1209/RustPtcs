fn main() {
    let x = add(2, 3);
    let bmi = calculate_bmi(70.5, 1.802);
    println!("The add result is: {}", x);
    println!("The BMI value is: {}",bmi);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}