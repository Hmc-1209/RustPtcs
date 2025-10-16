fn main() {
    hello_world();
    print_out_human_data("Bob", 175.5, 25);
}

fn hello_world() {
    println!("Hello, Rust!");
}

fn print_out_human_data(name: &str, height: f32, age: i32) {
    println!("The name is {}, height is {} cm and are now {} years old.", name, height, age);
}