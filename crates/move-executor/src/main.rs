use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {}

mod executor;

fn main() {
    let _ = Options::from_args();
    println!("Hello, world!");
}
