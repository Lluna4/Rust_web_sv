use std::cmp::Ordering;
use std::net::{TcpListener, TcpStream};
use std::io::{self, prelude::*, BufRead, BufReader};
use std::fs::File;
fn main() 
{
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Bind failed");

    for stream in listener.incoming()
    {
        let client = stream.unwrap();

        handle_client(client);
    }

}

fn handle_client(mut cli: TcpStream)
{
    let reader = BufReader::new(&mut cli);

    let req: Vec<_> = reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();

    let http_req = &req[0];
    println!("Request: {:#?}", req);
    println!("Exact request: {}", http_req);

    let http_req: Vec<&str> = http_req.split(' ').collect();

    match http_req[0] 
    {
        "GET" => handle_get(http_req, cli),
        _ => println!("Received a request that is not implemented yet!")
    }
}

fn handle_get(req: Vec<&str>, mut stream: TcpStream)
{
    if req[1].cmp("/") == Ordering::Equal
    {
        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n";
        
        stream.write_all(response.as_bytes()).unwrap();
        
        let mut file = File::open("hi.html").unwrap();
        io::copy(&mut file, &mut stream).expect("Sending file failed");
    }
}
