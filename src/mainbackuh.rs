use std::net;
use std::env;
use std::io;
use std::process;
use std::net::TcpStream;
use std::io::BufReader;
use std::fs;
use colored::Colorize;
use std::{
    io::{prelude::*, },
};
//mod utils;

/*
SIMPLE RUST WEB SERVER BY KODE!! -2024 yurrr

Made with the help of Rust One-Threaded simple web server example, also some stockoverflow
messages but mostly documentation research and, used ChatGPT to explain me a few errors
and how to fix them, also optimize, ALTHOUGH THE CODE IS NOT FULLY OPTIMIZED!
*/
fn main() {
    println!("{}", "
    ██╗  ██╗ ██████╗ ██████╗ ███████╗    ██╗    ██╗███████╗██████╗ ███████╗███████╗██████╗ ██╗   ██╗███████╗██████╗ 
    ██║ ██╔╝██╔═══██╗██╔══██╗██╔════╝    ██║    ██║██╔════╝██╔══██╗██╔════╝██╔════╝██╔══██╗██║   ██║██╔════╝██╔══██╗
    █████╔╝ ██║   ██║██║  ██║█████╗█████╗██║ █╗ ██║█████╗  ██████╔╝███████╗█████╗  ██████╔╝██║   ██║█████╗  ██████╔╝
    ██╔═██╗ ██║   ██║██║  ██║██╔══╝╚════╝██║███╗██║██╔══╝  ██╔══██╗╚════██║██╔══╝  ██╔══██╗╚██╗ ██╔╝██╔══╝  ██╔══██╗
    ██║  ██╗╚██████╔╝██████╔╝███████╗    ╚███╔███╔╝███████╗██████╔╝███████║███████╗██║  ██║ ╚████╔╝ ███████╗██║  ██║
    ╚═╝  ╚═╝ ╚═════╝ ╚═════╝ ╚══════╝     ╚══╝╚══╝ ╚══════╝╚═════╝ ╚══════╝╚══════╝╚═╝  ╚═╝  ╚═══╝  ╚══════╝╚═╝  ╚═╝
                                                                                                                    
    ".purple());
    print!("At which port do you want to run the server? ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let port: i32 = input.trim().parse().expect("Input is not a number");

    print!("Do you want to run it on the intranet? (Y/N): ");
    io::stdout().flush().unwrap();

    let mut input_bool = String::new();
    io::stdin().read_line(&mut input_bool).expect("Failed to read line");

    let address = match input_bool.trim().to_lowercase().as_str() {
        "y" => format!("0.0.0.0:{}", port),
        "n" => format!("127.0.0.1:{}", port),
        _ => format!("Only Y/N."),
        

    };
    if address == "Only Y/N." {
        println!("{address}");
       process::exit(69420);

    }




    env::set_var("RUST_BACKTRACE", "1");
    println!("Server is supposed to start at {}, also, BOMBOCLATTT!!!", port);
    let server = net::TcpListener::bind(format!("{address}"));
    
    
    for stream in server.expect("REASON").incoming() {
        println!("hi {:?}", stream);
        let x = handle_connection(stream.unwrap());

        if x.1 != "" {
        println!("Request is a {} on the {} endpoint, content sent is '{}'", x.0, x.1, x.2);
        } else {
            println!("{}", x.0);
        }
    }
   // Ok(());
    
    println!("Hello, world!");
}

fn global_speak(resp: String, mut connection: TcpStream) {
    Some(connection.write_all(resp.as_bytes()));
}

fn handle_connection(mut stream: TcpStream) -> (String, String, String) {
    let mut a = &stream;
    let buf_reader = BufReader::new(a); 
    //let req_type = String::new();
    let mut buffer = [0; 512];

    a.read(&mut buffer).unwrap();
	
    let request = String::from_utf8_lossy(&buffer[..]);

    let request_line = request.lines().next().unwrap_or(""); /* --> Well I improved this, 
                                                            I just removed a bootleg line of code an it works
                                                            so basically, dont fucking does this
                                                            it works somehow lol.
                                                             */
    let req_type = request_line
    .split_whitespace()
    .next()
    .unwrap_or("").to_string();

    let page_requested = request_line
    .split_whitespace()
    .nth(1)
    .unwrap_or("").to_string();

    let content = request.lines().last().unwrap_or("fuck this wtf is this shit you gave me bro?");

    let(status_line, contents) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", fs::read_to_string("src/hello.html").expect("wtf"))
    } else if request_line == "POST / HTTP/1.1" {
        ("HTTP/1.1 200 OK", String::from(format!(r#"<html>Um dis is akward....</html> why would send "{content}" my love?"#)))
    } else {
        ("HTTP/1.1 404 NOT FOUND", fs::read_to_string("src/404.html").expect("wtf"))
    };

    let content_len = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {content_len}\r\n\r\n{contents}");
    
    global_speak(response, stream);
    return (String::from(req_type), String::from(page_requested), String::from(content));

//println!("Request is a {}", req_type.to_string());


    /*
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
    */
    /*
  
    ****** 
    UNUSED CODE

    Substitute for the If part downwards,
    I made it myself w my knowledge lol, but I then proceeded to learn how to make it
    with variables instead of repeating if else multiple times or w match, although
    the match seems the most useful one if you are going to put a lot of pages

    ******
    let stat_200 = "HTTP/1.1 200 OK";

    match request_line {
      rline if rline == "GET / HTTP/1.1" => {
    let html_response = "<h1 style='color: orange;'>wagwanis</h1>";
    let html_response_len = html_response.len(); 
    let response = format!("{stat_200}\r\nContent-Length: {html_response_len}\r\n\r\n{html_response}");
    stream.write_all(response.as_bytes());
    //global_speak(response, stream);
      },

      _ => {
        let err400res = "<h1>OH CRAP IT IS ERROR 400!!!</h1><br><h3>BOOHOO bro, what were you even trying to find loll what the duckig duckk!?!?</h1>";
    let err400len = err400res.len();
    let response = format!("HTTP/1.1 400 NOT FOUND\r\nContent-Length: {err400len}\r\n\r\n{err400res}");
    stream.write_all(response.as_bytes());
    //global_speak(response, stream);
      },
    }
    
    END OF MATCH BLOCK
    */
    
    /*

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
