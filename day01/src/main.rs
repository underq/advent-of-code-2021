use std::env;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();

    let input_filename = day01::parse_config(&args).unwrap_or_else(|err| {
        println!("Problème rencontré lors de l'interprétation des arguments : {}", err);
        process::exit(1);
    });

    if let Err(e) = day01::run(input_filename) {
        println!("Erreur applicative : {}", e);

        process::exit(1);
    }
}
