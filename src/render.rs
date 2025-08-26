use crate::content::Content;
use crate::viewport::Viewport;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::text::Span;
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use unicode_width::UnicodeWidthChar;

#[derive(Copy, Clone)]
pub enum CharType {
    Normal,
    Invisible,
}

#[derive(Clone)]
pub struct RenderTerm {
    src_char: char,
    char_type: CharType,
    render_text: String,
    render_width: usize,
}

impl RenderTerm {
    pub fn from_char(src_char: char, tab_width: usize) -> Self {
        convert_char_to_render_term(src_char, tab_width)
    }

    pub fn to_span(&'_ self) -> Span<'_> {
        Span {
            content: self.render_text.to_owned().into(),
            style: Default::default(),
        }
    }
}

fn convert_char_to_render_term(c: char, tab_width: usize) -> RenderTerm {
    match c {
        '\t' => RenderTerm {
            src_char: c,
            char_type: CharType::Invisible,
            render_text: " ".repeat(tab_width).into(),
            render_width: tab_width,
        },
        _ => {
            if c.is_control() {
                let render_text = get_unicode_string(c);
                let render_width = render_text.len();
                RenderTerm {
                    src_char: c,
                    char_type: CharType::Invisible,
                    render_text,
                    render_width,
                }
            } else {
                RenderTerm {
                    src_char: c,
                    char_type: CharType::Invisible,
                    render_text: c.to_string(),
                    render_width: c.width().unwrap_or(1),
                }
            }
        }
    }
}

fn get_unicode_string(c: char) -> String {
    let code_point = c as u32;
    format!("U+{:X}", code_point)
}

/// 实际渲染的一行
pub struct RenderLine {
    /// terms
    pub terms: Vec<RenderTerm>,
    /// 源行索引
    pub src_line_index: usize,
}

pub struct RenderContent {
    pub lines: Vec<RenderLine>,
}

impl RenderContent {
    pub fn from_src_content_with_view_break(
        src_content: &Content,
        viewport: &Viewport,
        tab_width: usize,
    ) -> Self {
        let Viewport {
            anchor_col: _,
            anchor_row,
            width,
            height,
        } = *viewport;

        let start_row = if anchor_row >= src_content.lines.len() as i32 {
            src_content.lines.len().saturating_sub(1)
        } else {
            anchor_row.max(0) as usize
        };

        let end_row = if anchor_row + height as i32 > src_content.lines.len() as i32 {
            src_content.lines.len().saturating_sub(1)
        } else {
            (anchor_row + height as i32).max(0) as usize
        };

        let mut fill_rows = height as usize;
        let mut render_lines: Vec<RenderLine> = Vec::with_capacity(fill_rows);

        let mut terms = Vec::new();
        let mut curr_render_line_width_remain = width;

        for row in start_row..=end_row {
            let src_line = &src_content.lines[row];

            for c in src_line {
                let render_term = RenderTerm::from_char(*c, tab_width);
                let term_width = render_term.render_width;

                if curr_render_line_width_remain as i32 - term_width as i32 >= 0 {
                    terms.push(render_term);
                    curr_render_line_width_remain -= term_width as u16;
                } else {
                    // prepare new line
                    if fill_rows == 0 {
                        // 已经没有空间了
                        break;
                    }
                    fill_rows -= 1;
                    render_lines.push(RenderLine {
                        terms: terms.clone(),
                        src_line_index: row,
                    });
                    // 下一行
                    terms.clear();
                    curr_render_line_width_remain = width;
                    //
                    terms.push(render_term);
                    curr_render_line_width_remain -= term_width as u16;
                }
            }

            if fill_rows == 0 {
                break;
            }

            fill_rows -= 1;
            render_lines.push(RenderLine {
                terms: terms.clone(),
                src_line_index: row,
            });
            terms.clear();
            curr_render_line_width_remain = width;
        }

        RenderContent {
            lines: render_lines,
        }
    }
}

impl RenderContent {
    pub fn render(&self, f: &mut Frame, area: Rect) {
        let render_lines = &self.lines;
        let mut tui_lines: Vec<Line> = vec![];
        for render_line in render_lines {
            let RenderLine {
                terms,
                src_line_index: _,
            } = render_line;
            let spans: Vec<Span> = terms.iter().map(|term| term.to_span()).collect();
            tui_lines.push(Line::from(spans));
        }
        let paragraph = Paragraph::new(tui_lines);
        f.render_widget(paragraph, area);
    }
}

mod test {
    use super::*;

    #[test]
    fn test_render_content1() {
        let src_content = "hello\nworld.\niam\na\nvery\nhappy\nperson";
        let content = Content::from(src_content);

        assert_eq!(content.lines.len(), 7);
        assert_eq!(content.line_feed, Some("\n".into()));

        let viewport = Viewport {
            width: 3,
            height: 5,
            ..Default::default()
        };

        let render_content =
            RenderContent::from_src_content_with_view_break(&content, &viewport, 4);
        assert_eq!(render_content.lines.len(), 5);
        assert_eq!(render_content.lines[0].terms.len(), 3);
    }
    #[test]
    fn test_render_content2() {
        let src_content = r#"hello
你好，我的世界
im a happy boy"#;
        let content = Content::from(src_content);
        assert_eq!(content.lines.len(), 3);

        let viewport = Viewport {
            width: 3,
            height: 5,
            ..Default::default()
        };

        let render_content =
            RenderContent::from_src_content_with_view_break(&content, &viewport, 4);
        // ["hel", "lo", "你", "好", "，"]
        assert_eq!(render_content.lines.len(), 5);
        assert_eq!(render_content.lines[0].terms.len(), 3);
        assert_eq!(render_content.lines[1].terms.len(), 2);
        assert_eq!(render_content.lines[2].terms.len(), 1);
    }
}
