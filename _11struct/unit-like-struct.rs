struct Logger;

impl Logger {
    fn log(&self, msg: &str) {
        println!("[LOG] {}", msg);
    }
}

fn main() {
    let logger = Logger;
    logger.log("Program started!");
}