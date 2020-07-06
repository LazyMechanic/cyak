use super::event::Event;
use crossterm::event::KeyCode;

#[derive(Debug)]
pub struct Context {
    pub is_running: bool,
}

impl Default for Context {
    fn default() -> Self {
        Self { is_running: true }
    }
}

impl Context {
    pub fn handle_event(&mut self, e: Event) {
        match e {
            Event::Key(k) => match k.code {
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
                KeyCode::Char('q') => self.is_running = false,
                KeyCode::Char(_) => {}
                KeyCode::Null => {}
                KeyCode::Esc => {}
            },
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            Event::Tick => {}
        }
    }
}
