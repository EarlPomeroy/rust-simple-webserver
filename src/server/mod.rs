use std::net::TcpListener;

use crate::worker::Worker;

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
    base_path: String,
    last_worker_id: u32,
}

impl Server {
    pub fn new(base_path: String, bind_addr: String) -> Self {
        let listener = TcpListener::bind(bind_addr).expect("Could not create listener");

        Self {
            listener,
            base_path,
            last_worker_id: 0,
        }
    }

    pub fn listen(&mut self) {
        for conn in self.listener.incoming() {
            match conn {
                Ok(stream) => {
                    let mut worker =
                        Worker::new(stream, self.base_path.clone(), self.last_worker_id);
                    self.last_worker_id += 1;
                    worker.handle_request();
                }
                Err(_) => println!("Connection dropped"),
            }
        }
    }
}
