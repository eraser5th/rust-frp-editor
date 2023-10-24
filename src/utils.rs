pub fn centered_text(content: &str, text_width: usize) -> String {
    let padding = text_width.saturating_sub(content.len()) / 2;
    let spaces = " ".repeat(padding);
    let mut text = format!("{}{}", spaces, content);
    text.truncate(text_width);
    text
}
