extern crate structopt;
extern crate colored;

use structopt::StructOpt;
use std::{
    path,
    fs
};
use colored::*;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    /// Sound of the cat?
    message: String,

    #[structopt(short = "d", long = "dead")]
    /// Make the cat appear dead
    dead: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load a cat picture from the specified file
    catfile: Option<path::PathBuf>
}

fn print_cat(eye: &str) {
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ({eye}  {eye} )", eye = eye.green().bold());
    println!("   =( I )=");
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
    
    match &options.catfile {
        Some (path) => {
            let cat_template = fs::read_to_string(path)
                .expect(&format!("could not read file {:?}", path));
            let cat_img = cat_template.replace("{eye}", eye);
            println!("{}", &cat_img);
        },
        None => {
            print_cat(eye);
        }
    }
}
