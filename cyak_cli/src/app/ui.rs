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
    ($menu_name:ident, title: $title:literal, $(($display:literal, $id:ident) => |$siv:pat, $arg:pat, $ctx:pat, $preset_config:pat| $body:expr),+) => {
        pub enum $menu_name {
            $($id),+,
        }

        impl $menu_name {
            #[allow(dead_code)]
            pub fn name() -> &'static str {
                stringify!($menu_name)
            }

            #[allow(dead_code)]
            pub fn make(
                ctx: &std::rc::Rc<std::cell::RefCell<cyak_core::Context>>,
                preset_config: &std::rc::Rc<std::cell::RefCell<cyak_core::PresetConfig>>,
            ) -> impl cursive::view::View {
                use cursive::view::{Nameable, Scrollable, Resizable};
                use cursive::views::{SelectView, Dialog, ResizedView, LinearLayout};
                use cursive::align::HAlign;
                use cursive::Cursive;

                use cursive_aligned_view::Alignable;

                use std::cell::RefCell;
                use std::rc::Rc;

                use cyak_core::Context;
                use cyak_core::PresetConfig;

                Dialog::around(
                    SelectView::<$menu_name>::new()
                        .with_all(vec![$(($display, $menu_name::$id)),*])
                        .h_align(HAlign::Center)
                        .on_submit({
                            let ctx = Rc::clone(ctx);
                            let preset_config = Rc::clone(preset_config);
                            move |siv: &mut Cursive, item: &$menu_name| match item {
                            $(
                                $menu_name::$id => {
                                    (|$siv: &mut Cursive,
                                      $arg: &$menu_name,
                                      $ctx: &Rc<RefCell<Context>>,
                                      $preset_config: &Rc<RefCell<PresetConfig>>| $body)(siv, item, &ctx, &preset_config)
                                }
                            ),+,
                            }
                        })
                        .scrollable()
                        .scroll_x(true)
                        .scroll_y(true)
                        .align_top_center()
                    )
                    .title($title)
                    .max_size((50, 20))
                    .with_name($menu_name::name())
            }
        }
    };
}
