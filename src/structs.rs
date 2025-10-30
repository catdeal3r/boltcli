
pub const START_DATA: &str = "{\n\"model\": \"gpt-oss-120b\",\n\"stream\": false,\n\"max_tokens\": 2048,\n\"temperature\": 0.2,\n\"top_p\": 1,\"messages\": [\n{\n\"role\": \"system\",\n\"content\": \"";
pub const END_DATA: &str = "\"\n}\n\n]\n}";
pub const TITLE: &str = r#"
 ________  ________  ___       ___  _________  ________  _______   ________     
|\   ____\|\   __  \|\  \     |\  \|\___   ___\\_____  \|\  ___ \ |\   __  \    
\ \  \___|\ \  \|\  \ \  \    \ \  \|___ \  \_|\|___/  /\ \   __/|\ \  \|\  \   
 \ \_____  \ \   ____\ \  \    \ \  \   \ \  \     /  / /\ \  \_|/_\ \   _  _\  
  \|____|\  \ \  \___|\ \  \____\ \  \   \ \  \   /  /_/__\ \  \_|\ \ \  \\  \| 
    ____\_\  \ \__\    \ \_______\ \__\   \ \__\ |\________\ \_______\ \__\\ _\ 
   |\_________\|__|     \|_______|\|__|    \|__|  \|_______|\|_______|\|__|\|__|
   \|_________|
"#;

pub const COMMANDS: &str = r#"
`/init` - Setup a context based session.
`/reason` - Print the reason for an AI's output.
`/switch` - Switch to a different AI model temporarily.
`/exit` - Exit.

"#;
