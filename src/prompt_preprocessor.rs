
const PROMPT_PREPROCESSOR: &str = r#"Make sure your answer follow these
1. keep your answer as short as possible
2. do not need to explain anything
3. do not use ``
4. do not put the answer in the code snippet, give it in form of plaintext instead
5. if you do not think the input is a terminal command in natural language, instead of making things up, reply "do not know what you are talking about"
6. so your answer can either be a single terminal command or "do not know what you are talking about"
Platform: macos terminal

Help me to generate a terminal command that do the following:"#;

pub fn preprocess_prompt(input: &str) -> String {

    format!("{} {}", PROMPT_PREPROCESSOR, input)
}