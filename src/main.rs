use std::io::{self, Write};

use colored::*;
use termion::terminal_size;
use spinoff::{Spinner, spinners, Color};

pub mod utils;
pub mod structs;
pub mod conf;
pub mod parsing;

fn main() {
    let conf: conf::Config = conf::get_default_config();
    let mut model = conf.model.clone();
    let mut typing_mode = conf.typing_mode.clone();

    loop {
        let mut history: Vec<String> = Vec::new();
    
        utils::print_banner(model.clone(), typing_mode);

        let mut reason = String::new();
    
        loop {
            let mut input = String::new();

            let (_width, _height) = terminal_size().unwrap();

            let mut skin = termimad::MadSkin::default();
            utils::set_colours(&mut skin);
        
            let area = termimad::Area::full_screen();
             
            //termimad::print_inline(&utils::get_status_line(model.clone()));

            print!(" {}  ", ">".bold().blue());

            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut input)
                .unwrap();


            input = input.trim().to_string();


            if input == "/reason" {
                reason = format!("\n{}\n\n", reason);
                let formatted_content = termimad::FmtText::from(&skin, &reason, Some(area.width.into()));
                utils::print_via_typing(&formatted_content.to_string(), typing_mode);
                continue;
            }
        
            if input == "/typing" {
                typing_mode = !typing_mode;
                println!("\n{} Switched to typing mode: {}\n", "✓".bold().green(), typing_mode);
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

            if input == "/help" {
                termimad::print_inline(structs::COMMANDS);
                continue;
            }

            if input == "/new" {
                println!("\n{} Wiped history and started new session.", "✓".bold().green());
                break;
            }
       
            let mut result = String::new();

            let mut thinking_loading = Spinner::new(spinners::Dots3, "Thinking ...", Color::Blue);

            let mut instruct_input_ = String::new();

            instruct_input_.push_str(structs::INSTRUCTIONS);
            parsing::get_context(&mut instruct_input_);
            let _ = parsing::get_history(&mut instruct_input_, &history);
            instruct_input_.push_str(&input);

            let clean_input = serde_json::to_string(&instruct_input_).expect("serialization failed");
            let instruct_input = clean_input[1..clean_input.len() - 1].to_string();
        
            let request = &format!("{}{}{}{}{}", structs::START_DATA, model, structs::MIDDLE_DATA, instruct_input, structs::END_DATA);
                
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

            //thinking_loading.success("Finished.");
            thinking_loading.clear();

            reason = utils::get_reasoning(&result);

            let (raw_content, blocks) = parsing::extract_and_remove_blocks(&utils::get_content(&result), "OUTPUTFILE", "OUTPUTFILEEND");

            let content = &format!("\n{}\n", raw_content);

            history.push(input.clone());
            history.push(content.to_string());

            let formatted_content = termimad::FmtText::from(&skin, content, Some(area.width.into()));
            utils::print_via_typing(&formatted_content.to_string(), typing_mode);

            parsing::create_files_from_blocks(&blocks);
        }
    }
}

