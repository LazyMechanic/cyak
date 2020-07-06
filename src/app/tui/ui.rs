use crate::app::tui::ctx::Context;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::canvas::{Canvas, Map, MapResolution};
use tui::widgets::{Block, BorderType, Borders, List, ListState, Text};
use tui::Frame;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items<V>(items: V) -> StatefulList<T>
    where
        V: Into<Vec<T>>,
    {
        StatefulList {
            state: ListState::default(),
            items: items.into(),
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

pub fn draw<B: Backend>(f: &mut Frame<B>, ctx: &Context) {
    // let mut list = StatefulList::with_items(MENU);
    // let mut list_state = list.state;
    //
    // let app_size = f.size();
    // let horizontal = Layout::default()
    //     .direction(Direction::Horizontal)
    //     .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
    //     .split(app_size);
    //
    // let mock_menu = list.items.iter().map(|i| Text::raw(*i));
    // let menu = List::new(mock_menu)
    //     .block(Block::default().borders(Borders::ALL).title("Block"))
    //     .highlight_style(Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
    //     .highlight_symbol("> ");
    // f.render_stateful_widget(menu, horizontal[0], &mut list_state);
}
