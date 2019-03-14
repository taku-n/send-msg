use std::error::Error;
use std::fmt::Debug;
use std::io::prelude::*;
use std::net::TcpStream;

const ADDR_PORT: &str = "127.0.0.1:9841";

fn main() -> Result<(), Box<dyn Error>> {
    send_msg(2);
    send_msg(2.3);
    send_msg_i32(2);
    send_msg_i32(23);

    Ok(())
}

fn send_msg<T: Debug>(t: T) {
    println!("send_msg({:?}) begins.", t);

    let stream = TcpStream::connect(ADDR_PORT);

    let mut stream = match stream {
        Ok(t) => {
            println!("Connected to the server.");
            t
        },
        Err(e) => {
            panic!("Tried to connect but an error occured: {:?}", e);
        },
    };

    let msg = format!("{:?}", t);
    stream.write(msg.as_bytes());
    //write!(&mut stream, "{:?}", t);
    // Buggy with f64? Try to send 2.3. It sends just "2.".

    println!("send_msg() ends.");
}

fn send_msg_i32(x: i32) {
    println!("send_msg_i32({:?}) begins.", x);

    let stream: Result<std::net::TcpStream, std::io::Error>
            = TcpStream::connect(ADDR_PORT);

    let mut stream: std::net::TcpStream = match stream {
        Ok(t) => {
            println!("Connected to the server.");
            t
        },
        Err(e) => panic!("Tried to connect but an error occured: {:?}", e),
    };

    let msg: String = format!("{:?}", x);
    let result: Result<usize, std::io::Error> = stream.write(msg.as_bytes());

    match result {
        Ok(t) => println!("{} bytes were written.", t),
        Err(e) => panic!("Writing error: {:?}", e),
    };

    let result: Result<(), std::io::Error> = stream.flush();

    match result {
        Ok(t) => t,
        Err(e) => panic!("Flushing error: {:?}", e),
    };

    println!("send_msg_i32() ends.");
}
