use std::io::{self, Write};

use colored::Colorize;
use termion::terminal_size;
use spinoff::{Spinner, spinners, Color};

pub mod utils;
pub mod structs;
pub mod conf;

fn main() {
    let conf: conf::Config = conf::get_default_config();
    let mut model = conf.model.clone();
    
    print!("\nWelcome to");

    println!("{}", structs::TITLE.bold().blue());

    termimad::print_inline(structs::COMMANDS);

    let mut reason = String::new();
    
    loop {
        let mut input = String::new();
        termimad::print_inline(&format!("{} {} `{}`\n", std::env::current_dir().unwrap().display(), "─".bold().blue(), model));

        let (width, _height) = terminal_size().unwrap();

        for _i in 0..width {
            print!("─");
        }
        print!("\n");
        
        print!("{} ", ">".bold().green());

        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        
        for _i in 0..width {
            print!("─");
        }
        print!("\n");

        input = input.trim().to_string();

        if input == "/reason" {
            termimad::print_inline(&format!("\n{}\n\n", reason));
            continue;
        }

        if input == "/exit" {
            std::process::exit(0);
        }

        if input == "/switch" {
            print!("Model to switch to{} ", ":".bold().green());

            io::stdout().flush().unwrap();

            model.clear();

            io::stdin()
                .read_line(&mut model)
                .unwrap();

            model = model.trim().to_string();

            println!("\n{} Switched to {}\n", "✓".bold().green(), model);
            continue;
        }
       
        let mut result = String::new();

        let mut thinking_loading = Spinner::new(spinners::Dots2, "Thinking ...", Color::Blue);

        let request = &format!("{}{}{}{}{}", structs::START_DATA, model, structs::MIDDLE_DATA, input, structs::END_DATA);
        
        utils::send_ai_request(&"https://api.cerebras.ai/v1/chat/completions".to_string(),
            request,
            &mut result,
            &conf.key
        );


        /*
        println!("\n{}", request);
        println!("{}", result);
        */        
    
        if !utils::check_result_is_valid(&result) {
            thinking_loading.success("Finished.");
            continue;
        }

        thinking_loading.success("Finished.");

        reason = utils::get_reasoning(&result);
        
        let content = &format!("\n{}\n", utils::get_content(&result));

        let skin = termimad::MadSkin::default();
        let area = termimad::Area::full_screen();
        let formatted_content = termimad::FmtText::from(&skin, content, Some(area.width.into()));

        println!("{}", formatted_content);
    }
}

