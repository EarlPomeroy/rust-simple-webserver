mod config;
mod server;
mod worker;

fn main() {
    let config = config::Config::read();
    match config {
        Ok(config) => {
            let mut server = server::Server::new(config.get_base(), config.get_bind_address());
            println!("Server listening on {}", config.get_bind_address());
            loop {
                server.listen()
            }
        }
        Err(msg) => {
            println!("{}", msg);
        }
    }
}
