use std::io::Read;
use std::net::{TcpListener, TcpStream};
#[derive(Debug)]
struct Worker {
    stream: Option<TcpStream>,
    base_path: String,
}

impl Worker {
    fn new(base: String) -> Self {
        Self {
            stream: None,
            base_path: base,
        }
    }
}
#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
    worker: Worker,
}

impl Server {
    pub fn new(base_path: String, bind_addr: String) -> Self {
        let worker = Worker::new(base_path);
        let listener = TcpListener::bind(bind_addr).expect("Could not create listener");

        Self { listener, worker }
    }

    pub fn listen(&self) {
        for conn in self.listener.incoming() {
            match conn {
                Ok(stream) => self.handle_client(stream),
                Err(_) => println!("Connection dropped"),
            }
        }
    }

    fn handle_client(&self, mut stream: TcpStream) {
        println!("{:?}", stream);
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).unwrap();

        println!("{:?}", buffer);

        // Convert the buffer to a string
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received request:\n{}", request);
    }
}
