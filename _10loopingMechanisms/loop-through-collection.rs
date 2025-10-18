fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b: [char; 5] = ['1', '2', '3', '4', '5'];

    for num in a {
        println!("{}", num);
    }
    for chr in b {
        println!("{}", chr);
    }
}