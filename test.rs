extern crate simcolor;
use std::error::Error;
use simcolor::{Colorized};


fn main()-> Result<(), Box<dyn Error>> {
    // Foreground colors
    println!("My number is {:#x}!", 10.green());
    // Background colors
    println!("My number is not {}!", 4.on().red());
    println!("My number might be {}!", 4.black().on().yellow());
    // normal usage
    println!("{}", "green".green());
    println!("{}", "yellow".yellow());
    println!("{}", "blue".blue());
    println!("{}", "black".black().on().white());

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
    println!("{}", "magenta".bright().magenta());
    println!("{}", "white".white().bright());
    println!("{}", "cyan".cyan().bright());
    
    println!("{}{}", "\nBackgrounds\n".black().on().white(), "-------".yellow());
    println!("{}", "on green".on().green());
    println!("{}", "on yellow".on().yellow());
    println!("{}", "on blue".on().blue());
    println!("{}", "on black".white().on().black());

    println!("{}", "on red".on().red());
    println!("{}", "on magenta".on().magenta());
    println!("{}", "on white".on().white());
    println!("{}", "on cyan".on().cyan());


    println!("\nStyles\n-------");
    println!("{}", "underline".underline());
    println!("{}", "bold".bold());
    println!("{}", "italic".italic());
    println!("{}", "strikethrough".strikethrough());
    println!("{}", "reverse".reversed());
    println!("1{}3", "2".hidden());
    println!("{}", "blink".blink());
    println!("{}", "blink fast".blink_fast());
    println!("{} {}", "dimmed".bright().green().dimmed(), "text".bright().green());
    
    // foreground and background
    let red_on_white = "red on white".red().on().white();
    println!("{}", red_on_white);
    let world = "world".magenta().bright();
    let s = "hello".strikethrough();
    println!("{s}, {world}");
    let color = "𝕾𝖐𝖎𝖇𝖎𝖉𝖎 𝕿𝖔𝖎𝖑𝖊𝖙".underline();//.yellow();
    println!("{color}");
     let world = "world".bold();
    let hello_world = format!("Hello, {world}!");
    println!("{hello_world}");
    let hello_world = format!("Hello, {world}! and {color} in green").green();
    println!("{hello_world}");
    println!(
        "{:>8} {:<10} {}!",
        "it".green(),
        "works".blue().bold(),
        "great".bold().yellow()
    );
    Err("The test failed successfully".red())?
}