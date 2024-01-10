use std::net;
use std::env;
use std::io;
use std::process;
use std::fs;
use std::path::Path;
use std::net::TcpStream;
use std::io::BufReader;
use colored::Colorize;
use std::{
    collections::{HashMap},
    io::{prelude::*, },
};
mod utils;

/*
SIMPLE RUST WEB SERVER BY KODE!! -2024 yurrr

Made with the help of Rust One-Threaded simple web server example, also some stockoverflow
messages but mostly documentation research and, used ChatGPT to explain me a few errors
and how to fix them, also optimize, ALTHOUGH THE CODE IS NOT FULLY OPTIMIZED!
*/
fn main() {
    let mut file_names:  Vec<String>;
    println!("{} made by {} the og. \n ver: {} \n", "
    ██╗  ██╗ ██████╗ ██████╗ ███████╗    ██╗    ██╗███████╗██████╗ ███████╗███████╗██████╗ ██╗   ██╗███████╗██████╗ 
    ██║ ██╔╝██╔═══██╗██╔══██╗██╔════╝    ██║    ██║██╔════╝██╔══██╗██╔════╝██╔════╝██╔══██╗██║   ██║██╔════╝██╔══██╗
    █████╔╝ ██║   ██║██║  ██║█████╗█████╗██║ █╗ ██║█████╗  ██████╔╝███████╗█████╗  ██████╔╝██║   ██║█████╗  ██████╔╝
    ██╔═██╗ ██║   ██║██║  ██║██╔══╝╚════╝██║███╗██║██╔══╝  ██╔══██╗╚════██║██╔══╝  ██╔══██╗╚██╗ ██╔╝██╔══╝  ██╔══██╗
    ██║  ██╗╚██████╔╝██████╔╝███████╗    ╚███╔███╔╝███████╗██████╔╝███████║███████╗██║  ██║ ╚████╔╝ ███████╗██║  ██║
    ╚═╝  ╚═╝ ╚═════╝ ╚═════╝ ╚══════╝     ╚══╝╚══╝ ╚══════╝╚═════╝ ╚══════╝╚══════╝╚═╝  ╚═╝  ╚═══╝  ╚══════╝╚═╝  ╚═╝                                                                                                            
    ".purple(), "kodeine".purple(), "alpha v1.1 -- 2024-01-09".purple());
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
    if !Path::new("views").exists() {
        println!(r#"{}: Didnt find "views" folder, you need to use the "views" folder to place the .html files that are going to be displayed."#, "ERROR".red());
        process::exit(0);
    }
    let mut files_data = HashMap::new();
    
    let content404: String = String::from("404, non existent endpoint.");
    //let content404: String = String::from("404, non existent page.");
   match utils::get_files_in_dir("views") {
    Ok(file_names) => {
        /*loop {  --> Nice piece of code that fetches the code continuosly, every second
            print out the update thing, it needs an ampersand due to &file_names being
            continuosly moved
            
            sleep(1);*/
        for file_name in file_names {
            let mut file_content = fs::read_to_string(format!("views/{file_name}")).expect("File not found");
            if file_name == "index.html" {
                files_data.insert(String::from(""), file_content.clone());
            } else if file_name == "404.html" {
                files_data.insert(String::from("404"), file_content.clone());
                let content404 = file_content.clone();
            } else {
                files_data.insert(utils::remove_suffix(&file_name, ".html").to_string(), file_content.clone());
            }
            files_data.insert(utils::remove_suffix(&file_name, ".html").to_owned(), file_content);
            if file_name == "index.html" {
                println!("views/index.html --> FOR ENDPOINT {}", "/".green())
            } else if file_name == "404.html" {
                println!("views/404.html --> FOR {}", "ERROR 404 ".red())
            } else {
                println!("views/{file_name} --> FOR ENDPOINT {}", utils::remove_suffix(format!("/{}", &file_name).as_str(), ".html").to_owned().yellow());
            }
                //println!(r#"Content: "{file_content}""#);
            //Ok(files_data) => {.
            for (file_name, file_content) in files_data.iter() {
                let (mut fname, mut fcontent) = (file_name.to_string(), file_content.to_string());
                /*println!(r#"{} has for content -> "{}""#, fname, fcontent);
                println!("{fname} used for /{}", fname);  ''JUST TO SEE WHAT IS FETCHED FOR THIS FUNCTION''*/
            }
        }
    //}
    }
    Err(e) => (),
}
    
    
    for stream in server.expect("REASON").incoming() {
        println!("hi {:?}", stream);
        let x = handle_connection(stream.unwrap(), &files_data, content404.to_owned());

        /*if x.1 != "" {
        println!("Request is a {} on the {} endpoint, content sent is '{}'", x.0, x.1, x.2);
        } else {
            println!("{}", x.0);
        }*/
    }
   // Ok(());
   

    
    println!("Hello, world!");
}


fn global_speak(resp: String, mut connection: TcpStream) {
    Some(connection.write_all(resp.as_bytes()));
}

fn handle_connection(mut stream: TcpStream, files_data: &HashMap<String, String>, content404: String) /*-> (String, String, String)*/ {
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

    let binding = request_line
    .split_whitespace()
    .nth(1)
    .unwrap_or("").to_string();

    let page_requested = binding;
    let content = request.lines().last().unwrap_or("fuck this wtf is this shit you gave me bro?");
    
    


    /*println!("{:?}", files_data);
    println!("INB4 it goes mad lel, I hate debugging."); <- Used this to discover what was wrong loll, it is the 2nd time I write this commment, I deleted this braaa :Cry: 
    println!("{}", Some(&page_requested).unwrap());*/
    if let page = page_requested {
    let pagewithoutslash = utils::remove_prefix(&page, "/");
    //println!("{pagewithoutslash}");
    
    match files_data.get(pagewithoutslash) {
        
    None => {
        println!("Not found for page '{}'.", pagewithoutslash);
        let contents = String::from(content404);
        let content_len = contents.len();
       
       let status_line = "HTTP/1.1 200";
       let response = format!("{status_line}\r\nContent-Length: {content_len}\r\n\r\n{contents}");
       
       global_speak(response, stream);
        // or use println!("Not found for page '{:?}'.", page);
    },
    
    Some(value) => {
        println!("{} for page {}", "Request".yellow(), page.to_string());
        //println!("Supposedly it is {value} right? from page {}", page.to_string()); <- Just to see if it can actually fetch the correct value for the page requested
        let contents = String::from(format!("{value}"));
        //let contents = String::from("Working on the website right now nigga, xoxo❤️");
   
       let content_len = contents.len();
       
       let status_line = "HTTP/1.1 200";
       let response = format!("{status_line}\r\nContent-Length: {content_len}\r\n\r\n{contents}");
        global_speak(response, stream);
        

           },

}

        
        
    } else {
        println!("Page requested is None.");
    }
    
    let contents = String::from("Working on the website right now nigga, xoxo❤️");

    let content_len = contents.len();
    
    let status_line = "HTTP/1.1 200";
    let response = format!("{status_line}\r\nContent-Length: {content_len}\r\n\r\n{contents}");
    //global_speak(response, stream);
    //return (String::from(req_type), (&page_requested.expect("REASON")).to_string(), String::from(content));

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

