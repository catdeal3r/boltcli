
use colored::Colorize;
use curl::easy::{Easy, List};
use serde_json::Value;

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
    let values: Value = serde_json::from_str(raw_data).unwrap_or("Generation is too long and has been cut off.".into());
    values["choices"][0]["message"]["content"].as_str().unwrap_or(&format!("({}) Generated response is too long and has been cut off.", "Error".bold().red())).to_string()
}

pub fn get_reasoning(raw_data: &String) -> String {
    let values: Value = serde_json::from_str(raw_data).unwrap_or("Generation is too long and has been cut off.".into());
    values["choices"][0]["message"]["reasoning"].as_str().unwrap_or(&format!("({}) Generated response is too long and has been cut off.", "Error".bold().red())).to_string()
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
