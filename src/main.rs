use grep::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let config = Config::new(&args);
    let _ = match config {
        Ok(c) => grep::run(c),
        Err(e) => {
            println!("Error al parsear los argumentos: {}", e);
            return;
        }
    };
}
