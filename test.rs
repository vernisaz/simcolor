extern crate simcolor;
use simcolor::{Colorized};


fn main() {
    // Foreground colors
    println!("My number is {:#x}!", 10.green());
    // Background colors
    println!("My number is not {}!", 4.on().red());
    println!("My number might be {}!", 4.black().on().yellow());
    // normal usage
    println!("{}", "green".green());
    println!("{}", "yellow".yellow());
    println!("{}", "blue".blue());
    println!("{}", "black".black());

    println!("{}", "red".red());
    println!("{}", "magenta".magenta());
    println!("{}", "white".white());
    println!("{}", "cyan".cyan());

    println!("\nBrights\n-------");
    println!("{}", "green".green().bright());
    println!("{}", "yellow".yellow().bright());
    println!("{}", "blue".blue().bright());
    println!("{}", "black".black().bright());
    println!("{}", "red".red().bright());
    println!("{}", "magenta".magenta().bright());
    println!("{}", "white".white().bright());
    println!("{}", "cyan".cyan().bright());

    println!("\nStyles\n-------");
    println!("{}", "underline".underline());
    println!("{}", "bold".bold());
    println!("{}", "italic".italic());
    println!("{}", "strikethrough".strikethrough());
    println!("{}", "reverse".reversed());
    println!("1{}3", "2".hidden());
    println!("{}", "blink".blink());
    println!("{}", "blink fast".blink_fast());

    // foreground and background
    let red_on_white = "red on white".red().on().white();
    println!("{}", red_on_white);
    
    let s = "hello, world".strikethrough().bold().green();
    println!("{s}");
    let color = "strikethrough".strikethrough().green();
    println!("{color}")

}