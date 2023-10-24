use super::StatusLineComponent;

pub struct StatusLineModifiedComponent {}

impl StatusLineModifiedComponent {
    pub fn new(is_dirty: bool) -> StatusLineComponent {
        StatusLineComponent {
            text: if is_dirty { "(modified)" } else { "" }.to_string(),
        }
    }
}

pub struct StatusLineFileNameComponent {}

impl StatusLineFileNameComponent {
    pub fn new(file_name: Option<String>) -> StatusLineComponent {
        let text = if let Some(name) = file_name {
            let mut name = name.clone();
            name.truncate(20);
            name
        } else {
            "[No Name]".to_string()
        };

        StatusLineComponent { text }
    }
}

pub struct StatusLineLineIndicatorComponent {}

impl StatusLineLineIndicatorComponent {
    pub fn new(cursor_y: usize, document_len: usize) -> StatusLineComponent {
        let text = format!("{}/{}", cursor_y.saturating_add(1), document_len);
        StatusLineComponent { text }
    }
}
