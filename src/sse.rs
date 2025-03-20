use std::str::Lines;

pub struct SSELines<'a>(Lines<'a>);

impl<'a> From<Lines<'a>> for SSELines<'a> {
    fn from(lines: Lines<'a>) -> Self {
        Self(lines)
    }
}

impl<'a> Iterator for SSELines<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let line = self.0.next()?;

        if line.is_empty() {
            return self.next();
        }

        if line.starts_with("data: [DONE]") {
            return None;
        }

        Some(line.trim_start_matches("data: ").trim())
    }
}
