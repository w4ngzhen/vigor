mod content;
mod render;
mod viewport;

use crate::viewport::Viewport;
use color_eyre::Result;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::path::PathBuf;

fn main() -> Result<()> {
    let mut term = ratatui::init();

    let mut focus_editor = true;

    let content = content::Content::from(PathBuf::from("./assets/test.txt"));

    loop {
        term.draw(|f| {
            // render whole UI with black
            let rect = f.area();
            let viewport = Viewport::from(rect);
            let render_content =
                render::RenderContent::from_src_content_with_view_break(&content, &viewport, 4);
            render_content.render(f, rect);
        })?;

        let event = ratatui::crossterm::event::read()?;

        match event {
            Event::Key(key) if !matches!(key.kind, KeyEventKind::Press) => {
                ();
            }
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::F(1), _) => {
                    focus_editor = !focus_editor;
                }
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    break;
                }
                _ => {}
            },
            _ => {}
        }
    }

    ratatui::restore();

    Ok(())
}
