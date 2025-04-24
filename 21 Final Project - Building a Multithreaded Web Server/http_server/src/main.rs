use std::{
  fs,
  io::{prelude::*, BufReader, BufRead},
  net::{TcpListener, TcpStream},
  thread,
  time::Duration,
};

use http_server::ThreadPool;

fn main() {
  const PORT: u16 = 7878;

  let listener = match TcpListener::bind(format!("127.0.0.1:{PORT}")) {
    Ok(result) => {
      println!("Web Server Started on port: {PORT}");
      result
    },
    Err(err) => {
      println!("Error to start server - Exception: {}", err);
      return
    }
  };

  let pool = ThreadPool::new(4);

  for stream in listener.incoming().take(2) {
    let stream = stream.unwrap();

    pool.execute(|| {
      handle_connection(stream);
    })
  }

  println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
  let buf_reader = BufReader::new(&stream);

  let request_line = buf_reader.lines()
    .next()
    .unwrap()
    .unwrap();

  let content_type = "Content-Type: application/json";

  let (status_line, filename) = match &request_line[..] {
    "GET / HTTP/1.1" => ("200 OK", "200.json"),
    "GET /sleep HTTP/1.1" => {
      thread::sleep(Duration::from_secs(5));
      ("200 OK", "200.json")
    },
    _ => ("404 NOT FOUND", "404.json"),
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