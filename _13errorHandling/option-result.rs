fn divide_fnc_option(x: f32, y: f32) -> Option<f32> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn divide_fnc_result(x: f32, y: f32) -> Result<f32, String> {
    if y == 0.0 {
        Err("Cannot divide by 0".to_string())
    } else {
        Ok(x / y)
    }
}

fn main() {
    match divide_fnc_option(5.0, 3.0) {
        None => println!("Cannot divide by 0"),
        Some(x) => println!("Some number: {}", x),
    }
    match divide_fnc_option(5.0, 0.0) {
        None => println!("Cannot divide by 0"),
        Some(x) => println!("Some number: {}", x),
    }

    match divide_fnc_result(0.5, 0.1) {
        Err(e) => println!("Error: {}", e),
        Ok(x) => println!("Result: {}", x),
    }
    match divide_fnc_result(0.5, 0.0) {
        Err(e) => println!("Error: {}", e),
        Ok(x) => println!("Result: {}", x),
    }
}