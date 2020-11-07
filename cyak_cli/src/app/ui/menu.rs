use super::Ui;

pub trait Menu {
    type View: cursive::view::View;
    fn name() -> &'static str;
    fn make(ui: &std::rc::Rc<std::cell::RefCell<Ui>>) -> Self::View;
}

/// # Example
///
/// ```rust
/// use std::path::PathBuf;
/// use cyak_core::ProjectConfig;
///
/// menu! {
///     MainMenu,
///     ("Ok", Ok) => |siv, item, ctx| {
///         siv.add_layer(PresetMenu::make(ctx))
///     },
///     ("Exit", Exit) => |siv, item, ctx| {
///         std::process::exit(0);
///     }
/// }
///
/// menu! {
///     PresetMenu,
///     ("Ahoj", Ahoj) => |siv, item, ctx| {
///         siv.add_layer(Dialog::text("Ahoj").dismiss_button("ahoj"));
///     },
///     ("Back", Back) => |siv, item, ctx| {
///         siv.pop_layer();
///     }
/// }
///
/// fn main() {
///     let ctx = cyak_core::Context {
///         project_dir: PathBuf::from("/path/to/project"),
///         preset_dir: PathBuf::from("/path/to/preset"),
///         git: true,
///         license: None,
///         project_config: ProjectConfig::default(),
///     };
///     let mut siv = cursive::default();
///     siv.add_layout(MainMenu::make())
///     siv.run();
/// }
/// ```
#[macro_export]
macro_rules! menu {
    ($menu_name:ident
     ,title: $title:literal
     ,elements: {$($el_display:literal => |$el_siv:pat, $el_ui:pat| $el_body:expr),+}
     $(,buttons: {$($bt_display:literal => |$bt_siv:pat| $bt_body:expr),+})?
     $(,dismiss_button: $dbt_display:literal)?) => {
        pub struct $menu_name;

        impl Menu for $menu_name {
            type View = cursive::views::NamedView<cursive::views::ResizedView<cursive::views::Dialog>>;

            #[allow(dead_code)]
            fn name() -> &'static str {
                stringify!($menu_name)
            }

            #[allow(dead_code, unreachable_patterns, clippy::redundant_closure_call)]
            fn make(ui: &std::rc::Rc<std::cell::RefCell<Ui>>) -> Self::View {
                use cursive::view::{Nameable, Scrollable, Resizable};
                use cursive::views::{SelectView, Dialog};
                use cursive::align::HAlign;
                use cursive::Cursive;

                use cursive_aligned_view::Alignable;

                use std::cell::RefCell;
                use std::rc::Rc;

                Dialog::around(
                    SelectView::new()
                        .with_all_str(vec![$($el_display),*])
                        .h_align(HAlign::Center)
                        .on_submit({
                            let ui = Rc::clone(ui);
                            move |siv: &mut Cursive, item: &str| match item {
                            $(
                                $el_display => {
                                    (|$el_siv: &mut Cursive,
                                      $el_ui: &Rc<RefCell<Ui>>| $el_body)(siv, &ui)
                                }
                            ),+,
                                _ => unreachable!(),
                            }
                        })
                        .scrollable()
                        .scroll_x(true)
                        .scroll_y(true)
                        .align_top_center()
                    )
                    .title($title)
                    $(
                    .h_align(HAlign::Center)
                    $(
                    .button($bt_display, |$bt_siv: &mut Cursive| $bt_body)
                    )+
                    )?
                    $(
                    .dismiss_button($dbt_display)
                    )?
                    .max_size((50, 20))
                    .with_name($menu_name::name())
            }
        }
    };
}
