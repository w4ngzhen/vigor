use ratatui::prelude::Rect;

pub struct Viewport {
    pub anchor_col: i32,
    pub anchor_row: i32,
    pub width: u16,
    pub height: u16,
}

impl From<Rect> for Viewport {
    fn from(value: Rect) -> Self {
        Self {
            anchor_col: 0,
            anchor_row: 0,
            width: value.width,
            height: value.height,
        }
    }
}
