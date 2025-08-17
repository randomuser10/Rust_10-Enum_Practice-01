
#[derive(Debug)]
enum IpAddress {
    V4,
    V6,
}



fn main() {
    let four = IpAddress::V4;
    let six = IpAddress::V6;

    println!("The value of IpAddress4 is {:?}.",four);
    println!("The value of IPAddress6 is {:?}.", six);
}
