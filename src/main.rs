extern crate structopt;
extern crate colored;

use structopt::StructOpt;
use colored::*;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    /// Sound of the cat?
    message: String,

    #[structopt(short = "d", long = "dead")]
    /// Make the cat appear dead
    dead: bool
}

fn main() {
    let options = Options::from_args();
    let message = options.message;
    let eye = if options.dead { "x" }  else { "o" };

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.");
    }

    println!("{}", message.bright_yellow()
        .underline()
        .on_purple());
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ({eye}  {eye} )", eye = eye.green().bold());
    println!("   =( I )=");
}
