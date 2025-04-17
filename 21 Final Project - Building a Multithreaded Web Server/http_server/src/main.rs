use std::{
  fs,
  io::{prelude::*, BufReader, BufRead},
  net::{TcpListener, TcpStream},
};

fn main() {
  const PORT: u16 = 7878;

  let listener = match TcpListener::bind(format!("127.0.0.1:{PORT}")) {
    Ok(result) => {
      println!("Web Server Started on port: {PORT}");
      result
    },
    Err(err) => {
      println!("Error to start server - message {}", err);
      return
    }
  };

  for stream in listener.incoming() {
    let stream = stream.unwrap();
    handle_connection(stream);
  }
}

fn handle_connection(mut stream: TcpStream) {
  let buf_reader = BufReader::new(&stream);

  let request_line = buf_reader.lines()
    .next()
    .unwrap()
    .unwrap();

  let content_type = "Content-Type: application/json";

  let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
    ("200 OK", "200.json")
  } else {
    ("404 NOT FOUND", "404.json")
  };

  let content = fs::read_to_string(filename).unwrap();
  let length = content.len();

  let response = format!("HTTP/1.1 {status_line}\r
Content-Length: {length}\r
{content_type}\r
\r
{content}
");

  stream.write_all(response.as_bytes()).unwrap();
}