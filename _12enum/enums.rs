fn main() {
    #[derive(Debug)]
    enum IPAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home_ip: IPAddr = IPAddr::V4(127, 0, 0, 1);
    let loopback_ip: IPAddr = IPAddr::V6(String::from("::1"));

    println!("Home IP: {:?}", home_ip);
    println!("Loopback IP: {:?}", loopback_ip);
}
