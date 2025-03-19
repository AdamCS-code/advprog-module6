use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread, 
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
   
    let http_request : Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    if let Some(first_line) = http_request.first() {
        let parts: Vec<&str> = first_line.split_whitespace().collect();

        if parts.len() > 1 {

            let path = parts[1];

            println!("Requested path: {}", path);

            let (status_line, filename) = match path {
                "/" => ("HTTP/1.1 200 OK", "static/hello.html"),
                "/sleep" => {
                    thread::sleep(Duration::from_secs(10));
                    ("HTTP/1.1 200 OK", "static/hello.html")
                }
                _ => ("HTTP/1.1 404 NOT FOUND", "static/404.html"),
            };

            let contents = fs::read_to_string(filename).unwrap();

            let length = contents.len();
    
            let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}"); 

            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}
