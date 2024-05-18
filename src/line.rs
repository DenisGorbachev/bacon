use {
    crate::*,
    lazy_regex::regex_captures,
    std::path::PathBuf,
};

/// A report line
#[derive(Debug)]
pub struct Line {
    /// the index among items
    /// (all lines having the same index belong to
    /// the same error, warning, or test item)
    pub item_idx: usize,

    pub line_type: LineType,

    pub content: TLine,
}

impl Line {
    /// If the line is a title, get its message
    pub fn title_message(&self) -> Option<&str> {
        match self.line_type {
            LineType::Title(_) => self
                .content
                .strings
                .get(1)
                .map(|ts| ts.raw.as_str())
                .map(|s| s.trim_start_matches(|c: char| c.is_whitespace() || c == ':')),
            _ => None,
        }
    }

    /// If the line is normal. get its messages
    pub fn context(&self) -> Option<String> {
        match self.line_type {
            LineType::Normal => Some(
                self.content
                    .strings
                    .iter()
                    .map(|ts| ts.raw.as_str())
                    .collect::<Vec<&str>>()
                    .join(""),
            ),
            _ => None,
        }
    }

    /// Return the location as given by cargo
    /// It's usually relative and may contain the line and column
    pub fn location(&self) -> Option<&str> {
        match self.line_type {
            LineType::Location => {
                // the location part is a string at end like src/truc:15:3
                // or src/truc
                self.content
                    .strings
                    .last()
                    .and_then(|ts| regex_captures!(r"(\S+)$", ts.raw.as_str()))
                    .map(|(_, path)| path)
            }
            _ => None,
        }
    }
    /// Return the absolute path to the error/warning/test location
    pub fn location_path(
        &self,
        mission: &Mission,
    ) -> Option<PathBuf> {
        let location_path = self.location()?;
        let mut location_path = PathBuf::from(location_path);
        if !location_path.is_absolute() {
            location_path = mission.workspace_root.join(location_path);
        }
        Some(location_path)
    }
}

impl WrappableLine for Line {
    fn content(&self) -> &TLine {
        &self.content
    }
    fn prefix_cols(&self) -> usize {
        self.line_type.cols()
    }
}
