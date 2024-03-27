mod cli;

fn main() {
    let cli: cli::Cli = clap::Parser::parse();

    println!("Hello, world!");
}
