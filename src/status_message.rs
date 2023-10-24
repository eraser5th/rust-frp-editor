use std::time::Instant;

pub struct StatusMessage {
    pub text: String,
    pub time: Instant,
}

impl StatusMessage {
    pub fn from(message: &str) -> Self {
        Self {
            time: Instant::now(),
            text: message.to_string(),
        }
    }

    pub fn set_status(&mut self, message: &str) {
        self.text = message.to_string();
    }
}
