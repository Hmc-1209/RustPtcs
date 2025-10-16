fn main() {
    let x = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };
	println!("Total price: {}", x);
}
