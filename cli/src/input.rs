use std::collections::VecDeque;

use console::{style, Style};
use dialoguer::theme::ColorfulTheme;
use dialoguer::History;

pub struct InputHistory {
    max: usize,
    history: VecDeque<String>,
}

impl Default for InputHistory {
    fn default() -> Self {
        InputHistory {
            max: 10,
            history: VecDeque::new(),
        }
    }
}

impl<T: ToString> History<T> for InputHistory {
    fn read(&self, pos: usize) -> Option<String> {
        self.history.get(pos).cloned()
    }

    fn write(&mut self, val: &T) {
        if self.history.len() == self.max {
            self.history.pop_back();
        }
        self.history.push_front(val.to_string());
    }
}

pub fn generate_input_theme() -> ColorfulTheme {
    ColorfulTheme {
        defaults_style: Style::new().for_stderr().cyan(),
        prompt_style: Style::new().for_stderr().bold(),
        prompt_prefix: style("?".to_string()).for_stderr().yellow(),
        prompt_suffix: style(">".to_string()).for_stderr().black().bright(),
        success_prefix: style("✔".to_string()).for_stderr().white(),
        success_suffix: style("·".to_string()).for_stderr().black().bright(),
        error_prefix: style("✘".to_string()).for_stderr().red(),
        error_style: Style::new().for_stderr().red(),
        hint_style: Style::new().for_stderr().black().bright(),
        values_style: Style::new().for_stderr().black().bright(),
        active_item_style: Style::new().for_stderr().cyan(),
        inactive_item_style: Style::new().for_stderr(),
        active_item_prefix: style("❯".to_string()).for_stderr().green(),
        inactive_item_prefix: style(" ".to_string()).for_stderr(),
        checked_item_prefix: style("✔".to_string()).for_stderr().green(),
        unchecked_item_prefix: style("⬚".to_string()).for_stderr().magenta(),
        picked_item_prefix: style("❯".to_string()).for_stderr().green(),
        unpicked_item_prefix: style(" ".to_string()).for_stderr(),
    }
}
