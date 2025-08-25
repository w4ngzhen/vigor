use crate::content::Content;
use crate::viewport::Viewport;
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Widget};
use ratatui::text::Span;
use ratatui::widgets::Paragraph;
use unicode_width::UnicodeWidthChar;

pub enum CharType {
    Normal,
    Invisible,
}

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

pub struct RenderLine {
    pub terms: Vec<RenderTerm>,
    pub break_pos: Vec<usize>,
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
            anchor_col,
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

        let mut render_lines: Vec<RenderLine> = Vec::new();
        for row in start_row..=end_row {
            let mut render_line = RenderLine {
                terms: Vec::new(),
                break_pos: Vec::new(),
            };

            let mut idx = 0;
            let mut line_width_remain = width;

            let line = &src_content.lines[row];

            line.iter().for_each(|c| {
                let render_term = RenderTerm::from_char(*c, tab_width);
                let render_width = render_term.render_width;
                render_line.terms.push(render_term);

                if line_width_remain as i32 - (render_width as i32) < 0 {
                    render_line.break_pos.push(idx);
                    line_width_remain = width - render_width as u16;
                } else {
                    line_width_remain -= render_width as u16;
                }
                idx += 1;
            });

            let break_pos_len = render_line.break_pos.len();
            render_lines.push(render_line);
            if row + break_pos_len > end_row {
                break;
            }
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
            let RenderLine { terms, break_pos } = render_line;
            if break_pos.is_empty() {
                let spans: Vec<Span> = terms.iter().map(|term| term.to_span()).collect();
                tui_lines.push(Line::from(spans));
            }
        }
        let paragraph = Paragraph::new(tui_lines);
        f.render_widget(paragraph, area);
    }
}
