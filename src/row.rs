use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

pub struct Row {
    string: String,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect()
    }

    pub fn len(&self) -> usize {
        self.string[..].graphemes(true).count()
    }

    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }
}
