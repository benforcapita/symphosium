fn main() {
    let result = symphosium::cli::parse(std::env::args().skip(1)).and_then(symphosium::cli::run);
    match result {
        Ok(message) => println!("{message}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
