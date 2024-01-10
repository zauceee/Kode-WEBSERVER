use std::net::TcpListener;
use std::net::TcpStream;
use std::io::BufReader;
use std::{
    io::{prelude::*, },
};

pub let mut StreamVar: TcpStream = None;
pub fn connect() {
    let port: u32 = 8080;
    println!("Server is supposed to start at {}, also, BOMBOCLATTT!!!", port);
    let server = TcpListener::bind("127.0.0.1:8080");
    
    
    for stream in server.expect("REASON").incoming() {
        static StreamVar: TcpStream = stream;  
        println!("hi {:?}", stream);
        handle_connection(stream.unwrap());
        StreamVar = stream.unwrap();
    }
   // Ok(());
    
    println!("Hello, world!");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    println!("{}", request_line);
    /*let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);*/
    let stat_200 = "HTTP/1.1 200 OK";

    match request_line {
      rline if rline == "GET / HTTP/1.1" => {
    let html_response = "<h1 style='color: orange;'>wagwanis</h1>";
    let html_response_len = html_response.len(); 
    let response = format!("{stat_200}\r\nContent-Length: {html_response_len}\r\n\r\n{html_response}");
    //stream.write_all(response.as_bytes()).unwrap();
    global_speak(response);
      },

      _ => {
        let err400res = "<h1>OH CRAP IT IS ERROR 400!!!</h1><br><h3>BOOHOO bro, what were you even trying to find loll what the duckig duckk!?!?</h1>";
    let err400len = err400res.len();
    let response = format!("HTTP/1.1 400 NOT FOUND\r\nContent-Length: {err400len}\r\n\r\n{err400res}");
    stream.write_all(response.as_bytes()).unwrap();
      },
    }/*

    ******
     DEPRECATED CODE
     Well made on the same day, demoted on the same ducking day lmfaooo,
     one of mew few functional and useful rust code, well I got help from https://doc.rust-lang.org/book/ch20-01-single-threaded.html 
     which is really helpful!
     I look forward onto learing more about Rust and its features
     im going to see if I can handle post req now, since most of these are GET's

    ******
    if request_line == "GET / HTTP/1.1" {
    let stat_200 = "HTTP/1.1 200 OK";
    let html_response = "<h1 style='color: orange;'>wagwanis</h1>";
    let html_response_len = html_response.len(); 
    let response = format!("{stat_200}\r\nContent-Length: {html_response_len}\r\n\r\n{html_response}");
    stream.write_all(response.as_bytes()).unwrap();
} else {
    let err400res = "<h1>OH CRAP IT IS ERROR 400!!!</h1><br><h3>BOOHOO bro, what were you even trying to find loll what the duckig duckk!?!?</h1>";
    let err400len = err400res.len();
    let response = format!("HTTP/1.1 400 NOT FOUND\r\nContent-Length: {err400len}\r\n\r\n{err400res}");
    stream.write_all(response.as_bytes()).unwrap();
}*/
}
