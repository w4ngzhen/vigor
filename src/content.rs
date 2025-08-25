use std::path::{Path, PathBuf};
use unicode_width::UnicodeWidthChar;

#[derive(Default)]
pub struct Content {
    pub lines: Vec<Vec<char>>,
    line_feed: Option<String>,
}

impl Content {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            line_feed: None,
        }
    }

    /// "hello,世界" -> 8
    pub fn row_char_len(&self, row_idx: usize) -> Option<usize> {
        self.lines.get(row_idx).map(|line| line.len())
    }

    /// "hello,世界" -> 10
    pub fn row_char_unicode_width(&self, row_idx: usize) -> Option<usize> {
        if let Some(line) = self.lines.get(row_idx) {
            let width = line
                .iter()
                .fold(0, |prev, ch| prev + ch.width().unwrap_or_default());
            Some(width)
        } else {
            None
        }
    }

    pub fn get_row(&self, row_idx: usize) -> Option<&Vec<char>> {
        self.lines.get(row_idx)
    }
}

impl From<&str> for Content {
    fn from(value: &str) -> Self {
        // 存储识别到的文本换行符
        let mut line_feed: Option<String> = None;
        // 最终存储的每一行，每一个字符
        let mut lines = vec![];
        // temp_line，用于下面的遍历识别过程中，存储识别到的每一行
        let mut temp_line = vec![];
        for ch in value.chars() {
            match ch {
                '\n' => {
                    let curr_line_feed: String;
                    if temp_line.ends_with(&['\r']) {
                        temp_line.pop(); // 末尾是'\r'，则移除
                        curr_line_feed = "\r\n".into(); // 且知道当前这一行是的换行是"\r\n"
                    } else {
                        curr_line_feed = "\n".into(); // 当前行是以"\n"作为换行符的
                    }
                    // 首次出现的换行符
                    if line_feed.is_none() {
                        line_feed = Some(curr_line_feed)
                    }
                    // 存放到多行容器中
                    lines.push(temp_line);
                    temp_line = vec![];
                }
                c => temp_line.push(c),
            }
        }
        // 完成遍历以后，temp_line还有字符，此时是最后一行
        if temp_line.len() > 0 {
            lines.push(temp_line);
        }
        // 最后一行也没识别出换行符，则使用系统默认换行符
        if line_feed.is_none() {
            line_feed = Some(match std::env::consts::OS {
                "windows" => "\r\n".into(),
                _ => "\n".into(),
            })
        }
        Self { lines, line_feed }
    }
}

impl From<PathBuf> for Content {
    fn from(value: PathBuf) -> Self {
        std::fs::read_to_string(value)
            .unwrap_or(String::new())
            .as_str()
            .into()
    }
}

mod test {
    use crate::content::Content;
    use ratatui::prelude::Span;

    #[test]
    fn test_from() {
        let content: Content = "Hello\n".into();
        assert_eq!(content.lines.len(), 1);
        assert_eq!(content.line_feed, Some("\n".into()));

        let content: Content = "Hello\nworld\r\n世界".into();
        assert_eq!(content.lines.len(), 3);
        assert_eq!(content.line_feed, Some("\n".into()));

        let content: Content = "Hello\r\nworld\n".into();
        assert_eq!(content.lines.len(), 2);
        assert_eq!(content.line_feed, Some("\r\n".into()));

        let content: Content = "Hello\nworld\n\n\n\n".into();
        assert_eq!(content.lines.len(), 5); // 注意这里是5行而不是6行
        assert_eq!(content.line_feed, Some("\n".into()));
    }

    #[test]
    fn test_row_char_len() {
        let content = Content::from("hello,世界");
        let len = content.row_char_len(0).unwrap();
        assert_eq!(len, 8);
    }
    #[test]
    fn test_row_char_unicode_width() {
        let content = Content::from("hello,世界");
        let len = content.row_char_unicode_width(0).unwrap();
        assert_eq!(len, 10);
    }

    #[test]
    fn test_row_char_unicode_width_same_with_span() {
        let str = "hello,世界";
        let content = Content::from(str);
        let span = Span::from(str);
        assert_eq!(content.row_char_unicode_width(0).unwrap(), span.width());
    }
}
