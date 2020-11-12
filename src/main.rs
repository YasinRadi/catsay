extern crate structopt;
extern crate colored;

use colored::*;
use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;
use std::{
    path,
    fs
};


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

fn print_cat(eye: ColoredString) {
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ({eye}  {eye} )", eye = eye);
    println!("   =( I )=");
}

fn main() -> Result<(), ExitFailure> {
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
                .with_context(|_| format!("could not read file {:?}", path))?;
            let cat_img = cat_template.replace("{eye}", eye);
            println!("{}", &cat_img);
        },
        None => {
            print_cat(eye.green().bold());
        }
    }

    Ok(())
}
