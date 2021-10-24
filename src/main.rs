use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;

fn main() {
    // listen to the port 8080
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Starting the server...");
    // Handle multiple connections from clients
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // Handle the connection from client
        match  handle_connection(stream){
            Ok(_) => { println!("Message received and processed successfully.")}
            Err(_) => {println!("Failed to process message.")}
        }
    }
}

// The connection handler function to process request message from client
fn handle_connection(mut stream: TcpStream) -> Result<(), &'static str>{
    println!("Message received from client...");
    // define the read buffer as 1024 bytes
    let mut buffer = [0; 1024];

    // read message from the stream to the buffer
    let num_read = stream.read(&mut buffer).unwrap();

    // if buffer is empty, return Err;
    if num_read == 0 {return Err("invalid request");}

    // convert the type of string to vector
    let get = b"hello world".to_vec();

    // convert the message to string
    let s = match str::from_utf8(&get) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    // pattern match for the request header
    match str::from_utf8(&buffer).unwrap().contains(&s) {
        true => {
            // print the message from client if the message is hello world
            println!("Message received: {}", &s);
            // response with message in get
            let contents = s.to_string();
            println!("Sending echo message: {}", &contents);
            let response = format!(
                "{}",
                contents
            );
            // write the message back to the stream
            stream.write(response.as_bytes()).unwrap();
            // clear the stream buffer
            stream.flush().unwrap();}
        false => {
            // some other request
            let status_line = "HTTP/1.1 404 NOT FOUND";
            let contents ="Unable to process the message.";

            let response = format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                status_line,
                contents.len(),
                contents
            );
            // write the messag to client
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();}
    }

    return Ok(());
}
