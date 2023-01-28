/*
System commands executed in rust
*/

use ::std::process::Command;
use ::yansi::{Color, Paint, Style};

// Defining the colors shit here

fn main() {
    println!(
        "
        {}
    ",
        Paint::blue("Executing Commands via yansi Rust")
    );
    bn_1();
    s_1();
}

// Banner Code here
fn bn_1() {
    let di_1 = Style::new(Color::Green).bold().bg(Color::Magenta);
    println!(
        "{}",
        di_1.paint(
            "
System Commands with Pipes
**************************"
        )
    )
}

// System Commands block
fn s_1() {
    let mut exa_o1 = Command::new("exa");
    exa_o1.args(["-la", "--icons"]).status().expect("Bastard");
    print!("{}", "{exa_o1:?}",);
}

fn s_2() {
    let mut exa_o1 = Command::new("exa");
    exa_o1.args(["-la", "--icons"]).status().expect("Bastard");
    print!("{exa_o1:?}");
}
