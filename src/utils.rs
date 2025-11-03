
use colored::Colorize;
use curl::easy::{Easy, List};
use serde_json::Value;
use termion::terminal_size;
use termimad::crossterm::style;
use std::{
    io::{self, Write},
    thread::sleep, time::Duration
};
use crate::parsing;

pub fn send_ai_request(url: &String, data: &String, r_output: &mut String, key: &str) {
    let mut request = Easy::new();

    request.url(url).unwrap();

    let mut headers = List::new();
    headers.append("Content-Type: application/json").unwrap();
    headers.append(&format!("Authorization: Bearer {}", key)).unwrap();

    request.http_headers(headers).unwrap();
    
    request.post(true).unwrap();
    request.post_fields_copy(data.as_bytes()).unwrap();

    let mut transfer = request.transfer();
    transfer.write_function(|data| {
        *r_output = String::from_utf8_lossy(data).to_string();
        Ok(data.len())
    }).unwrap();

    transfer.perform().unwrap();
}

pub fn get_content(raw_data: &String) -> String {
    let values: Value = serde_json::from_str(raw_data).unwrap_or("gjasfd".into());
    values["choices"][0]["message"]["content"].as_str().unwrap_or(&format!("({}) Failed to parse content. Check log for details.", "Error".bold().red())).to_string()
}

pub fn get_reasoning(raw_data: &String) -> String {
    let values: Value = serde_json::from_str(raw_data).unwrap_or("gjskhdafg".into());
    values["choices"][0]["message"]["reasoning"].as_str().unwrap_or(&format!("({}) Failed to parse content. Check log for details.", "Error".bold().red())).to_string()
}

pub fn check_result_is_valid(response: &String) -> bool {
    if response == r#"{"message":"Wrong API Key","type":"invalid_request_error","param":"api_key","code":"wrong_api_key"}"# {
        println!("\n({}): Invalid API key.", colored::Colorize::red("Error"));
        std::process::exit(1);
    } else if response.contains("does not exist or you do not have access to it.\",\"type\":\"not_found_error\",\"param\":\"model\",\"code\":\"model_not_found\"}") {
        println!("\n({}): Invalid model name.", colored::Colorize::red("Error"));
        return false;
    }
    
    true
}


pub fn get_status_line(model: String) -> String {
    
    let (width, _height) = terminal_size().unwrap();

    let mut status_line_str = String::new();
    
    let current_dir: String = std::env::current_dir().unwrap().display().to_string();
    let model_str = format!("{} `{}`", "Using".bold().blue(), model);

    status_line_str.push_str(&current_dir);

    let current_dir_len = current_dir.len() as i16;
    let model_str_len = model_str.len() as i16;
    let width_len = width as i16;
    
    let padding_amount = width_len - model_str_len - current_dir_len + 13;
    
    for _i in 0..padding_amount {
        status_line_str.push(' ');
    }
    status_line_str.push_str(&model_str);
    status_line_str.push('\n');
    
    status_line_str
}

pub fn print_via_typing(content: &String, typing_mode: bool) {
    if typing_mode {
        for c in content.chars() {
            io::stdout().flush().unwrap();
            print!("{}", c);
            sleep(Duration::from_millis(1));
        }
    } else {
        println!("{}", content);
    }
}

pub fn set_colours(skin: &mut termimad::MadSkin) {
    skin.set_fg(style::Color::Rgb { r: 210, g: 210, b: 210 }); // orange-ish              
                                                                             
    skin.bold.set_fg(style::Color::Rgb { r: 255, g: 255, b: 255 });                     
                                                                            
    skin.italic.set_fg(style::Color::Rgb { r: 180, g: 180, b: 180 });                   
                                                                             
    skin.code_block.set_fg(style::Color::Rgb { r: 200, g: 200, b: 200 });
    skin.code_block.set_bg(style::Color::Rgb { r: 30, g: 30, b: 30 }); 
}

pub fn print_banner(model_name: String, typing_bool: bool) {

    let greeting = format!("Welcome back.");
    let line = "│".bold().blue();
    let line_dim = "│".bold().dimmed();
    let block = "█▌".bold().blue();
    let mut model = format!("{} {}", "· Using".bold().dimmed(), model_name.dimmed());
    for _i in 0..15 - model_name.len() {
        model.push(' ');
    }

    let typing;

    if typing_bool {
        typing = format!("{} {} {} ", "·".bold().dimmed(), "Typing is".dimmed(), "on".bold().dimmed());
    } else {
        typing = format!("{} {} {}", "·".bold().dimmed(), "Typing is".dimmed(), "off".bold().dimmed());
    }

    let mut project_name = String::new();
    if parsing::get_context(&mut project_name) == 1 {
        project_name = "# Uninitialized".to_string();
    }

    project_name = project_name.lines().nth(1).unwrap_or("# Uninitialized").to_string();

    let project_formatted;
    let project_status;
    let project_hint;

    if project_name == "# Uninitialized".to_string() {
        project_formatted = format!("{}                   ", &project_name.bold().dimmed());
        project_name = "# Uninitialized                   ".to_string();
        project_status = format!("{} {} {}         {line}", "·".bold().dimmed(), "No".bold().dimmed(), "project is initialized".dimmed());
        project_hint = format!("{} {} {} {}  {line}", "·".bold().dimmed(), "Use".dimmed(), "/init".bold().dimmed(), "to start a new project".dimmed());
    } else {
        if project_name.len() < 23 {
            let mut padding = String::new();
            for _i in 0..24 - project_name.len() {
                padding.push(' ');
            }
            
            project_formatted = format!("{project_name}{padding}");
            
            project_name = format!("{project_name}{padding}");
        } else {
            project_formatted = format!("{}", project_name);
        }

        let mut spacing = String::new();
        for _i in 0..project_name.len() - 24 {
            spacing.push(' ');
        }
        project_status = format!("{} {} {} {spacing} {line}", "·".bold().dimmed(), "Project is".dimmed(), "initialized".bold().dimmed());
        project_hint = format!("                         {spacing} {line}");
    }

    let mut end_line = "─────────────".to_string();

    for _i in 0..project_name.len() {
        end_line.push('─');
    }
    end_line.push('╮');

    let mut bottom_line = "╰───────────────────────────────────".to_string();

    for _i in 0..project_name.len() {
        bottom_line.push('─');
    }
    bottom_line.push('╯');

    let mut line_padded = "    ".to_string();

    for _i in 0..project_name.len() {
        line_padded.push(' ');
    }
    line_padded = format!("{}{}", line_padded, "│".bold().blue());
    

    println!("\n{} BoltCli {} {}", "╭────────".bold().blue(), "v0.8".dimmed(), &end_line.bold().blue());
    
    println!("{line}                              {line_dim}{line_padded}
{line}  {block}   {greeting}          {line_dim}  {project_formatted}  {line}
{line}   {}  {model}{line_dim}  {project_status}
{line}  {block}   {typing}        {line_dim}  {project_hint}
{line}                              {line_dim}{line_padded}",
        "▐█".bold().blue());

    println!("{}\n", &bottom_line.bold().blue());

    termimad::print_inline("Type `/help` to show all commands.\n\n");
}
