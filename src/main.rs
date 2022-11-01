use clap::Parser;

mod fetcher;
mod function;
mod solution;

#[derive(Parser)]
struct Cli {
    /// Function to use
    function: String,
    /// Argument based on first command
    arg: i64,
}

fn main() {
    println!("Welcome to reetcode");
    let args = Cli::parse();
    let generate = String::from("generate");
    match args.function {
        generate => {
            let id = args.arg;
            function::generate::generate_template(id);
        }
        _ => {
            println!("Unregonized function");
        }
    }
}
