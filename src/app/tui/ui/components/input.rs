use super::Component;
use crossterm::event::{KeyCode, KeyModifiers};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::{Block, BorderType, Borders};
use tui::Frame;

use super::Drawable;
use super::Error;

use crate::app::tui::event::Event;
use tui::buffer::Buffer;

pub struct Input {
    title:        Option<String>,
    default_text: Option<String>,
    text:         String,
    style:        Style,
    rect:         Rect,
    limit:        Limit,

    cursor_pos: usize,

    focused: bool,
    visible: bool,
}

impl Input {
    pub fn new<S: Into<String>>(
        title: Option<S>,
        default_text: Option<S>,
        style: Style,
        rect: Rect,
        limit: Limit,
    ) -> Self {
        let title = title.map(|t| t.into());
        let default_text = default_text.map(|d| d.into());

        Self {
            title,
            default_text,
            text: "".to_string(),
            style,
            rect,
            limit,
            cursor_pos: 0,
            focused: false,
            visible: true,
        }
    }

    /// Set `title`.
    pub fn set_title<S: Into<String>>(&mut self, title: Option<S>) {
        self.title = title.map(|s| s.into());
    }

    /// Force set `text` and move cursor to the beginning. Return `Error` if
    /// new text overflow limit.
    pub fn set_text<S: Into<String>>(&mut self, text: S) -> Result<(), Error> {
        let text = text.into();
        if let Limit::Limited(l) = self.limit {
            if l > text.len() {
                return Error::TextLimitOverflow(l, text).fail();
            }
        }

        self.text = text;
        self.cursor_pos = 0;

        Ok(())
    }

    /// Set `rect`.
    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    /// Set `limit`. If `limit == Limit::Unlimited` then max `text` size is unlimited
    /// (is still limited by usize). Return `Error` if new limit less then `text.len()`.
    pub fn set_limit(&mut self, limit: Limit) -> Result<(), Error> {
        if let Limit::Limited(l) = limit {
            if self.text.len() > l {
                return Error::TextLimitOverflow(l, self.text.clone()).fail();
            }
        }

        self.limit = limit;

        Ok(())
    }

    /// Clear the `text`.
    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor_pos = 0;
    }

    /// Get the `text`.
    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    /// Move the cursor to `0`.
    pub fn home_cursor(&mut self) {
        self.cursor_pos = 0;
    }

    /// Move the cursor to `text.len()`.
    pub fn end_cursor(&mut self) {
        self.cursor_pos = self.text.len();
    }

    /// Move the cursor right one char.
    pub fn inc_cursor(&mut self) {
        if let Some(p) = self.next_char_position() {
            self.cursor_pos = p;
        }
    }

    /// Move the cursor left one char.
    pub fn dec_cursor(&mut self) {
        if let Some(p) = self.prev_char_position() {
            self.cursor_pos = p;
        }
    }

    /// Get the position of the next char, or, if the cursor points
    /// to the last char, the `text.len()`.
    /// Returns None when the cursor is already at `text.len()`.
    fn next_char_position(&self) -> Option<usize> {
        if self.cursor_pos >= self.text.len() {
            return None;
        }
        let mut index = self.cursor_pos.saturating_add(1);
        while index < self.text.len() && !self.text.is_char_boundary(index) {
            index += 1;
        }
        Some(index)
    }

    /// Get the position of the previous char, or, if the cursor points
    /// to the first char, the `0`.
    /// Returns None when the cursor is already at `0`.
    fn prev_char_position(&self) -> Option<usize> {
        if self.cursor_pos == 0 {
            return None;
        }
        let mut index = self.cursor_pos.saturating_sub(1);
        while index > 0 && !self.text.is_char_boundary(index) {
            index -= 1;
        }
        Some(index)
    }

    /// Remove symbol from `text` left relative to cursor and decrement it.
    fn backspace(&mut self) {
        if self.cursor_pos > 0 {
            self.dec_cursor();
            self.text.remove(self.cursor_pos);
        }
    }

    /// Remove symbol from `text` under cursor.
    fn delete(&mut self) {
        if self.text.len() > self.cursor_pos {
            self.text.remove(self.cursor_pos);
        }
    }
}

impl Drawable for Input {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, _rect: Rect) -> Result<(), Error> {
        if !self.visible {
            return Ok(());
        }

        Ok(())
    }
}

impl Component for Input {
    fn event(&mut self, e: Event) -> Result<(), Error> {
        if !self.focused || !self.visible {
            return Ok(());
        }

        if let Event::Key(k) = e {
            let is_ctrl = k.modifiers.contains(KeyModifiers::CONTROL);
            match k.code {
                KeyCode::Backspace => self.backspace(),
                KeyCode::Enter => {}
                KeyCode::Left => self.dec_cursor(),
                KeyCode::Right => self.inc_cursor(),
                KeyCode::Home => self.home_cursor(),
                KeyCode::End => self.end_cursor(),
                KeyCode::Delete => self.delete(),
                KeyCode::Char(c) if !is_ctrl => {
                    self.text.insert(self.cursor_pos, c);
                    self.inc_cursor();
                }
                _ => {}
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

#[derive(Debug, Eq, PartialEq)]
pub enum Limit {
    Limited(usize),
    Unlimited,
}
