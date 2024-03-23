use clap::Parser;
#[derive(Parser)]
struct Cli {
    pattern: String,
}
fn main() {
    let ans: &str = "mentalsupport";
    let args: Cli= Cli::parse();
    if &args.pattern == ans {
        println!("Your dissertation has been submitted already")
    }
}