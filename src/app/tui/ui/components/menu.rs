use crossterm::event::{KeyCode, KeyModifiers};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::Frame;

use crate::app::tui::event::Event;
use crate::app::tui::ui::Error;

use super::Component;
use super::Drawable;

pub struct Menu {
    focused: bool,
    visible: bool,
}

impl Drawable for Menu {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect) -> Result<(), Error> {
        if !self.visible {
            return Ok(());
        }

        Ok(())
    }
}

impl Component for Menu {
    fn event(&mut self, e: Event) -> Result<(), Error> {
        if !self.focused || !self.visible {
            return Ok(());
        }

        if let Event::Key(k) = e {
            let is_ctrl = k.modifiers.contains(KeyModifiers::CONTROL);
            match k.code {
                KeyCode::Backspace => {}
                KeyCode::Enter => {}
                KeyCode::Left => {}
                KeyCode::Right => {}
                KeyCode::Up => {}
                KeyCode::Down => {}
                KeyCode::Home => {}
                KeyCode::End => {}
                KeyCode::PageUp => {}
                KeyCode::PageDown => {}
                KeyCode::Tab => {}
                KeyCode::BackTab => {}
                KeyCode::Delete => {}
                KeyCode::Insert => {}
                KeyCode::F(_) => {}
                KeyCode::Char(_) => {}
                KeyCode::Null => {}
                KeyCode::Esc => {}
            }
        }

        Ok(())
    }

    fn focus_status(&self) -> bool {
        self.focused
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }

    fn visible_status(&self) -> bool {
        self.visible
    }

    fn hide(&mut self) {
        self.visible = false;
    }

    fn show(&mut self) {
        self.visible = true;
    }
}
