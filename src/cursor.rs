use crate::content::Content;
use unicode_width::UnicodeWidthChar;

#[derive(Default)]
pub struct Cursor {
    x: usize,
    y: usize,
}

impl Cursor {
    pub fn calc_col_at_char_pos(
        cursor_x: usize,
        content: &Content,
        row_idx: usize,
    ) -> Option<usize> {
        if let Some(row) = content.get_row(row_idx) {
            if row.len() <= cursor_x {
                None
            } else {
                let mut char_x_idx = 0;
                let mut col: usize = 0;
                // cursor的实际col，应该为[0..cursor_x-1]的所有char的unicode width之和
                while char_x_idx < cursor_x {
                    let ch = row[char_x_idx];
                    col += ch.width().unwrap_or(1);
                    char_x_idx += 1;
                }
                Some(col)
            }
        } else {
            None
        }
    }
}

mod test {
    use crate::content::Content;
    use crate::cursor::Cursor;

    #[test]
    fn test_calc_col_at_char_pos() {
        let content = Content::from("hello,世界");
        let cursor_col = Cursor::calc_col_at_char_pos(0, &content, 1);
        assert_eq!(cursor_col, None);
        let cursor_col = Cursor::calc_col_at_char_pos(10, &content, 0);
        assert_eq!(cursor_col, None);
        let cursor_col = Cursor::calc_col_at_char_pos(0, &content, 0);
        assert_eq!(cursor_col, Some(0));
        let cursor_col = Cursor::calc_col_at_char_pos(6, &content, 0);
        assert_eq!(cursor_col, Some(6));
        let cursor_col = Cursor::calc_col_at_char_pos(7, &content, 0);
        assert_eq!(cursor_col, Some(8));
    }
}
