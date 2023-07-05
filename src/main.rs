mod config;
mod server;

fn main() {
    let config = config::Config::read();
    match config {
        Ok(config) => {
            let server = server::Server::new(config.get_base(), config.get_bind_address());
            println!("Server listening on {}", config.get_bind_address());
            println!("{:?}", server);
            loop {
                server.listen()
            }
        }
        Err(msg) => {
            println!("{}", msg);
        }
    }
}
