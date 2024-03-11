use colored::*;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("No hay suficientes argumentos");
        }
        let query = args[2].clone();
        let filename = args[1].clone();
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) {
    let content = fs::read_to_string(config.filename).expect("algo ocurrió al leer el archivo");
    let results = search(&config.query, &content);

    println!(
        "Se encontraron: {} coindicencias",
        results.len().to_string().green()
    );

    for line in results {
        let parts: Vec<&str> = line.split(&config.query).collect();
        println!(
            "{}",
            parts.join(&config.query.on_yellow().black().to_string())
        );
    }
}

pub fn run_list_all_in_folder(config: Config) {
    if let Ok(entries) = fs::read_dir("./") {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                println!("{}", path.to_str().unwrap());
                if path.is_file() {
                    run(Config {
                        query: config.query.clone(),
                        filename: path.to_str().unwrap().to_string(),
                    });
                }
            }
        }
    } else {
        eprintln!("Error al leer el directorio");
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
