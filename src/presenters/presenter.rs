use inquire::{Confirm, Text};

pub struct Presenter {}

impl Presenter {
    #[allow(dead_code)]
    pub fn prompt_get_string(
        message: String,
        help_prompt: String,
        default_value: String,
    ) -> String {
        Text::new(message.as_str())
            .with_help_message(help_prompt.as_str())
            .with_default(default_value.as_str())
            .prompt()
            .unwrap()
    }

    #[allow(dead_code)]
    pub fn prompt_get_u32(input: String) -> u32 {
        match input.chars().find(|character| character.is_numeric()) {
            Some(_) => input.as_str().trim().parse::<u32>().unwrap(),
            None => panic!("Error: Non-numeric value"),
        }
    }

    #[allow(dead_code)]
    pub fn prompt_get_confirmation(message: String) -> bool {
        Confirm::new(message.as_str())
            .with_default(false)
            .prompt()
            .unwrap_or_default()
    }
}
