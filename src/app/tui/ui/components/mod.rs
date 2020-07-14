pub mod input;
pub mod menu;

use tui::backend::Backend;
use tui::layout::Rect;
use tui::Frame;

use crate::app::tui::event::Event;
use crate::app::tui::ui::Error;

pub trait Drawable {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect) -> Result<(), Error>;
}

pub trait Component {
    /// Handle event
    fn event(&mut self, e: Event) -> Result<(), Error>;

    /// Get focus state
    fn focus_status(&self) -> bool;
    /// Focus this component
    fn focus(&mut self);
    /// Unfocus this component
    fn unfocus(&mut self);
    /// Focus/unfocus this component
    fn toggle_focus(&mut self) {
        self.set_focus(self.focus_status());
    }
    /// Focus/unfocus this component depending on param
    fn set_focus(&mut self, focus: bool) {
        if focus {
            self.focus();
        } else {
            self.unfocus();
        }
    }

    /// Get visible status
    fn visible_status(&self) -> bool;
    /// Hide component
    fn hide(&mut self);
    /// Show component
    fn show(&mut self);
    /// Show/hide this component
    fn toggle_visible(&mut self) {
        self.set_visible(self.visible_status());
    }
    /// Show/hide this component depending on param
    fn set_visible(&mut self, vis: bool) {
        if vis {
            self.show();
        } else {
            self.hide();
        }
    }
}
