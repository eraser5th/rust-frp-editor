use termion::event::Key;

use crate::Row;
use crate::Terminal;

#[derive(Default)]
pub struct Prompt {
    value: Row,
}

#[derive(PartialEq, Eq)]
pub enum PromptStatus {
    Aborted,
    InProgress,
    Committed,
}

impl Prompt {
    pub fn prompt(&mut self) -> Result<(PromptStatus, Option<String>), std::io::Error> {
        let status: PromptStatus = match Terminal::read_key()? {
            Key::Char('\n') => PromptStatus::Committed,
            Key::Esc => PromptStatus::Aborted,
            Key::Backspace => {
                self.value.delete(self.value.len());
                PromptStatus::InProgress
            }
            Key::Char(c) => {
                if !c.is_control() {
                    self.value.insert(self.value.len(), c);
                }
                PromptStatus::InProgress
            }
            _ => PromptStatus::InProgress,
        };

        let value = if status == PromptStatus::Aborted {
            None
        } else {
            Some(self.value.to_string())
        };

        Ok((status, value))
    }
}
