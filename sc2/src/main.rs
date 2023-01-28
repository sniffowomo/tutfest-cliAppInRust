/*
Second attempt - for piping system commands
*/

// Importing crates color and standard
use ::yansi::{Color, Paint, Style};
// Import crate for system command s
use ::std::process::*;
use std::vec;
// Cfonts crates for banner
use ::cfonts::*;

// Stop import warnings
#[warn(unused_imports)]

fn main() {
    cf1();
    println!(
        "{}",
        Paint::red("System Commands in Rust with pipes ")
            .blink()
            .bold()
            .underline()
    );
}

//cfonts setup banner

fn cf1() {
    say(Options {
        text: String::from("Hello"),
        font: Fonts::FontBlock,
        line_height: 0,
        colors: vec![Colors::RedBright],
        gradient: Vec::new(),
        ..Options::default()
    });
}
