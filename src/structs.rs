
pub const START_DATA: &str = "{\n\"model\": \""; 
pub const MIDDLE_DATA: &str = "\",\n\"stream\": false,\n\"max_tokens\": 2048,\n\"temperature\": 0.2,\n\"top_p\": 1,\"messages\": [\n{\n\"role\": \"system\",\n\"content\": \"";
pub const END_DATA: &str = "\"\n}\n\n]\n}";
pub const TITLE: &str = r#"

  █    ██████   ██████   ██    ████████  ██████ ██      ██ 
 ██     ██   ██ ██    ██  ██       ██    ██      ██      ██ 
 █ █    ██████  ██    ██  ██       ██    ██      ██      ██ 
  ██    ██   ██ ██    ██  ██       ██    ██      ██      ██ 
  █     ██████   ██████   ███████  ██     ██████ ███████ ██ 
"#;

pub const COMMANDS: &str = r#"
`/init`   → Setup a context based session.
`/reason` → Print the reason for an AI's output.
`/switch` → Switch to a different AI model temporarily.
`/typing` → Toggle whether the generated text is typed.
`/exit`   → Exit.

"#;
