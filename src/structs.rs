
pub const START_DATA: &str = "{\n\"model\": \""; 
pub const MIDDLE_DATA: &str = "\",\n\"stream\": false,\n\"max_tokens\": 2048,\n\"temperature\": 0.2,\n\"top_p\": 1,\"messages\": [\n{\n\"role\": \"system\",\n\"content\": \"";
pub const END_DATA: &str = "\"\n}\n]\n}";
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


pub const INSTRUCTIONS: &str = r#"You are replying to user's questions about coding, research, or any other topic.
If a user references a filename, the contents of the file will place at the top of the question.
E.g. If a user references doc.md, the top of the question will have this:
doc.md:
`
# Cool document content!
Blah blah blah.
`
For extra information about the project or question, there may be a section at the top of the file like this (for example):
CONTEXT:
`
# The coffee project - perfect brewing.

blah blah blah blah, blah blah.

## Subheading
blah blah:
- blah blah
`
and so on.
These files submitted by the user and CONTEXT may be arranged in any which way.
If the user asks to generate something into a file, simple end your response with 'OUTPUTFILE', then a newline, then the file name and contents.
E.g. If a user ask to generate a coffee tutorial into `coffee-tutorial.md`, output this at the end of the file:
OUTPUTFILE
coffee-tutorial.md
`
# Coffee Tutorial

blah blah blah ...
`

Important points:
- Do no reply to this message.
- Always output something to the user other than the OUTPUTFILE.
- If the user doesn't ask for a file output, do not create the OUTPUTFILE heading.
- If a user asks to edit a file, take that file input and output the edited version into the OUTPUTFILE heading.
- Never output anything after the OUTPUTFILE.
---
"#;
