/*
Second attempt - for piping system commands
*/

// Importing crates color and standard
use ::yansi::{Color, Paint, Style};
// Import crate for system command s
use ::std::process::*;
// Cfonts crates for banner
use ::cfonts::*;

// Stop import warnings
#[warn(unused_imports)]

fn main() {
    println!(
        "{}",
        Paint::red("System Commands in Rust with pipes ")
            .blink()
            .bold()
            .underline()
    );
    Cf_1();
}

//cfonts crate test

fn Cf_1() {
    say(Options {
        text: String::from("Hello"),
        font: Fonts::FontChrome,
        colors: Colors::BlueBright,
        ..Options::default()
    });
}
