
#[derive(Debug)]
enum IpAddress {
    V4(String),
    V6(String),
}
#[derive(Debug)]
enum Message{
    Quit,
    Move(i32, i32),
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self){
                    //
        
    }
}

fn main() {
    let four = IpAddress::V4(String::from("Hello World"));
    let six = IpAddress::V6(String::from("How are you?"));

    println!("The value of IpAddress4 is {:?}.",four);
    println!("The value of IPAddress6 is {:?}.", six);
    let m = Message::Write(String::from("Hello"));
    println!("{:?}", m);


}
