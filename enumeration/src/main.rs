fn main() {
    enum IpAddr {
        V4(String),
        V6(String),
    }
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from(":1"));

    // , it’s also easier to see another detail of how enums work: the name of each enum variant that we define also 
    // becomes a function that constructs an instance of the enum. That is, IpAddr::V4() is a function call that 
    // takes a String argument and returns an instance of the IpAddr type.
    println!("Hello, world!");
}
