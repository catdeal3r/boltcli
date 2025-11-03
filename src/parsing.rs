
use std::fs;
use spinoff::{Spinner, spinners, Color};

// yes I used ai
pub fn extract_and_remove_blocks<'a>(input: &'a str, start_marker: &str, end_marker: &str) -> (String, Vec<String>) {                                                           
    // The result we will return                                                       
    let mut cleaned = String::new();          // input without the blocks              
    let mut blocks: Vec<String> = Vec::new(); // collected block contents              
                                                                                       
    // Temporary buffer while we are inside a block                                    
    let mut current_block = String::new();                                             
                                                                                       
    // Simple state machine                                                            
    enum State {                                                                       
        Outside,                                                                       
        Inside,                                                                        
    }                                                                                  
    let mut state = State::Outside;                                                    
                                                                                       
    // Iterate over the lines, keeping the original newline characters.                
    // `lines()` strips the trailing `\n`, so we add it back manually.                 
    for (i, line) in input.lines().enumerate() {                                       
        // Preserve the original line ending (the last line may not have one)          
        let line_ending = if i + 1 == input.lines().count() && !input.ends_with('\n') {
            ""                                                                         
        } else {                                                                       
            "\n"                                                                       
        };                                                                             
                                                                                       
        match state {                                                                  
            State::Outside => {                                                        
                if line.trim() == start_marker {                                       
                    // We have hit the beginning of a block.                           
                    state = State::Inside;                                             
                    // Do **not** copy the start marker into `cleaned`.                
                } else {                                                               
                    // Normal line – keep it.                                          
                    cleaned.push_str(line);                                            
                    cleaned.push_str(line_ending);                                     
                }                                                                      
            }                                                                          
            State::Inside => {                                                         
                if line.trim() == end_marker {                                         
                    // End of the block – store it and go back to Outside.             
                    blocks.push(current_block.clone());                                
                    current_block.clear();                                             
                    state = State::Outside;                                            
                    // Do **not** copy the end marker into `cleaned`.                  
                } else {                                                               
                    // Still inside the block – accumulate the line.                   
                    current_block.push_str(line);                                      
                    current_block.push_str(line_ending);                               
                }                                                                      
            }                                                                          
        }                                                                              
    }                                                                                  
                                                                                       
    // If the input ends while we are still inside a block, treat the                  
    // collected text as a block (you could also decide to keep it).                   
    if let State::Inside = state {                                                     
        blocks.push(current_block);                                                    
    }                                                                                  
                                                                                       
    // Trim a possible trailing newline that `cleaned` may have acquired.              
    if cleaned.ends_with('\n') {                                                       
        cleaned.pop(); // remove the '\n'                                              
        if cleaned.ends_with('\r') {                                                   
            cleaned.pop(); // also remove '\r' on Windows line endings                 
        }                                                                              
    }                                                                                  
                                                                                       
    (cleaned, blocks)                                                                  
}    


pub fn create_files_from_blocks(blocks: &Vec<String>) {
    if blocks.is_empty() {
        return;
    }
    let mut file_loading = Spinner::new(spinners::Dots2, "Creating file ...", Color::Blue);
    
    for (_i, blk) in blocks.iter().enumerate() {
        
        let filename = blk.lines().next().unwrap_or("").to_string();
        let content = blk.lines().skip(1).collect::<Vec<_>>().join("\n");
        
        fs::write(&filename, &content).unwrap();
        file_loading.success(&format!("Saved to {}", filename));
        println!("");
    }
}


pub fn get_context(str: &mut String) -> u16 {

    if !std::path::Path::exists(std::path::Path::new("CONTEXT.md")) {
        return 1;
    }
    
    let mut context = "CONTEXT\n".to_string();
    context.push_str(&fs::read_to_string("CONTEXT.md").unwrap());
    context.push_str("\nCONTEXTEND\n---\n");

    str.push_str(&context);
    0
}


pub fn get_history(str: &mut String, his: &Vec<String>) {
    let mut history = "HISTORY\n".to_string();
    for s in his {
        history.push_str(&format!("{}\n\n", s));
    }
    history.push_str("HISTORYEND\n---\n");

    str.push_str(&history);
}
