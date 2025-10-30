use std::io::{self, Write};

use colored::Colorize;
pub mod utils;
pub mod structs;
pub mod conf;
use termion::terminal_size;

fn main() {
    let conf: conf::Config = conf::get_default_config();
    
    print!("Welcome to");

    println!("{}", structs::TITLE.bold().blue());

    termimad::print_inline(structs::COMMANDS);

    let mut reason = String::new();
    
    loop {
        let mut input = String::new();
        print!("{}\n", std::env::current_dir().unwrap().display());

        let (width, _height) = terminal_size().unwrap();

        for _i in 0..width {
            print!("─");
        }
        print!("\n");
        
        print!("{} ", ">".bold().green());

        // flush to show immediately
        io::stdout().flush().unwrap();

        // read input
        io::stdin()
            .read_line(&mut input)
            .unwrap();

        input = input.trim().to_string();

        if input == "/reason" {
            termimad::print_inline(&format!("\n{}\n\n", reason));
            continue;
        }

        if input == "/exit" {
            std::process::exit(0);
        }
       
        let mut result = String::new();

        utils::send_ai_request(&"https://api.cerebras.ai/v1/chat/completions".to_string(),
            &format!("{}{}{}", structs::START_DATA, input, structs::END_DATA),
            &mut result,
            &conf.key
        );

        utils::check_api_key_is_valid(&result);

        reason = utils::get_reasoning(&result);
        
        let content = &format!("\n{}\n", utils::get_content(&result));

        let skin = termimad::MadSkin::default();
        let area = termimad::Area::full_screen();
        let formatted_content = termimad::FmtText::from(&skin, content, Some(area.width.into()));

        println!("{}", formatted_content);
    }
}

