/// # Example
///
/// ```rust
///
///
/// crate::menu! {
///     CustomMenu,
///     title: { "Custom menu" },
///     items: {
///         |_ui| -> Result<_, Error> {
///             Ok(vec![
///                 ("Go to default select menu 1", "menu_1"),
///                 ("Go to default select menu 2", "menu_2"),
///                 ("Item 1", "item_1"),
///                 ("Item 2", "item_2"),
///                 ("Exit", "exit"),
///             ])
///         }
///     },
///     on_submit: {
///         |siv, item, ui| -> Result<(), Error> {
///             match item {
///                 "menu_1" => {
///                     siv.add_layer(DefaultSelectMenu1::make(ui)?);
///                 },
///                 "menu_2" => {
///                     siv.add_layer(DefaultSelectMenu2::make(ui)?);
///                 },
///                 "item_1" => {
///                     siv.add_layer(Dialog::text(format!("Submit {}", item)).dismiss_button("Close"));
///                 },
///                 "item_2" => {
///                     siv.add_layer(Dialog::text(format!("Submit {}", item)).dismiss_button("Close"));
///                 },
///                 "exit" => {
///                     siv.quit();
///                 },
///                 _ => unreachable!(),
///             }
///             Ok(())
///         }
///     },
///     on_select: {
///         |_siv, item, _ui| -> Result<(), Error> {
///             log::info!("Select {}", item);
///             Ok(())
///         }
///     },
///     buttons: {
///         "Another exit" => |siv, _ui| -> Result<(), Error> {
///             siv.quit();
///             Ok(())
///         }
///     },
///     size: { (50, 20) }
/// }
///
/// crate::menu! {
///     DefaultSelectMenu1,
///     title: { "Default select menu 1" },
///     items: { from_submit },
///     on_submit: {
///         "item_1" => |siv, item, _ui| -> Result<(), Error> {
///             siv.add_layer(Dialog::text(format!("Submit {}", item)).dismiss_button("Close"));
///             Ok(())
///         },
///         "item_2" => |siv, item, _ui| -> Result<(), Error> {
///             siv.add_layer(Dialog::text(format!("Submit {}", item)).dismiss_button("Close"));
///             Ok(())
///         }
///     },
///     on_select: {
///         "item_1" => |_siv, item, _ui| -> Result<(), Error> {
///             log::info!("Select {}", item);
///             Ok(())
///         },
///         "item_2" => |_siv, item, _ui| -> Result<(), Error> {
///             log::info!("Select {}", item);
///             Ok(())
///         }
///     },
///     buttons: {
///         "Back" => |siv, _ui| -> Result<(), Error> {
///             siv.pop_layer();
///             Ok(())
///         }
///     },
///     size: { default }
/// }
///
/// crate::menu! {
///     DefaultSelectMenu2,
///     title: { "Default select menu 2" },
///     items: {
///         |_ui| -> Result<Vec<_>, Error> {
///             Ok(vec![
///                 ("Item 1", "item_1"),
///                 ("Item 2", "item_2"),
///                 ("Item 3", "item_3"),
///                 ("Item 4", "item_4"),
///                 ("Item 5", "item_5"),
///                 ("Back",   "back"),
///             ])
///         }
///     },
///     on_submit: {
///         "item_1" => |siv, item, _ui| -> Result<(), Error> {
///             siv.add_layer(Dialog::text(format!("Submit {}", item)).dismiss_button("Close"));
///             Ok(())
///         },
///         "item_2" => |siv, item, _ui| -> Result<(), Error> {
///             siv.add_layer(Dialog::text(format!("Submit {}", item)).dismiss_button("Close"));
///             Ok(())
///         },
///         "item_3" => |siv, item, _ui| -> Result<(), Error> {
///             siv.add_layer(Dialog::text(format!("Submit {}", item)).dismiss_button("Close"));
///             Ok(())
///         },
///         "item_4" => |siv, item, _ui| -> Result<(), Error> {
///             siv.add_layer(Dialog::text(format!("Submit {}", item)).dismiss_button("Close"));
///             Ok(())
///         },
///         "item_5" => |siv, item, _ui| -> Result<(), Error> {
///             siv.add_layer(Dialog::text(format!("Submit {}", item)).dismiss_button("Close"));
///             Ok(())
///         },
///         "back" => |siv, _item, _ui| -> Result<(), Error> {
///             siv.pop_layer();
///             Ok(())
///         }
///     },
///     on_select: {
///         "item_1" => |_siv, item, _ui| -> Result<(), Error> {
///             log::info!("Select {}", item);
///             Ok(())
///         },
///         "item_2" => |_siv, item, _ui| -> Result<(), Error> {
///             log::info!("Select {}", item);
///             Ok(())
///         },
///         "item_3" => |_siv, item, _ui| -> Result<(), Error> {
///             log::info!("Select {}", item);
///             Ok(())
///         },
///         "item_4" => |_siv, item, _ui| -> Result<(), Error> {
///             log::info!("Select {}", item);
///             Ok(())
///         },
///         "item_5" => |_siv, item, _ui| -> Result<(), Error> {
///             log::info!("Select {}", item);
///             Ok(())
///         },
///         "back" => |siv, item, _ui| -> Result<(), Error> {
///             log::info!("Select {}", item);
///             Ok(())
///         }
///     },
///     buttons: {},
///     size: { default }
/// }
///
/// fn main() {
///     use std::rc::Rc;
///     use std::cell::RefCell;
///
///     let ui = Rc::new(RefCell::new(Ui{...}));
///     let mut siv = cursive::default();
///     siv.add_layout(CustomMenu::make(&ui))
///     siv.run();
/// }
/// ```

#[macro_export]
macro_rules! menu {
    ($menu_name:ident
     ,title: { $title:literal }
     ,items: { |$items_ui:pat| -> Result<$items_ok:ty, Error> $items_body:expr }
     ,on_submit: { $($on_submit_display:literal => |$on_submit_siv:pat, $on_submit_item:pat, $on_submit_ui:pat| -> Result<(), Error> $on_submit_body:expr),+ }
     ,on_select: { $($($on_select_display:literal => |$on_select_siv:pat, $on_select_item:pat, $on_select_ui:pat| -> Result<(), Error> $on_select_body:expr),+)? }
     ,buttons: { $($bt_display:literal => |$bt_siv:pat, $bt_ui:pat| -> Result<(), Error> $bt_body:expr),* }
     ,size: { default }
    ) => {
        $crate::menu!{
            $menu_name
            , title: { $title }
            , items: {
                |$items_ui| -> Result<$items_ok, Error> $items_body
            }
            , on_submit: {
                |siv, item, ui| -> Result<(), Error> {
                    match item {
                    $(
                        $on_submit_display => {
                            (|$on_submit_siv: &mut cursive::Cursive,
                              $on_submit_item: &_,
                              $on_submit_ui: &std::rc::Rc<std::cell::RefCell<$crate::app::ui::Ui>>| -> Result<(), $crate::app::ui::Error> { $on_submit_body })(siv, item, &ui)?
                        }
                    )+
                        _ => unreachable!(),
                    }
                    Ok(())
                }
            }
            , on_select: {
                |siv, item, ui| -> Result<(), Error> {
                    $(
                    match item {
                    $(
                        $on_select_display => {
                            (|$on_select_siv: &mut cursive::Cursive,
                              $on_select_item: &_,
                              $on_select_ui: &std::rc::Rc<std::cell::RefCell<$crate::app::ui::Ui>>| -> Result<(), $crate::app::ui::Error> { $on_select_body })(siv, item, &ui)?
                        }
                    )+
                        _ => unreachable!(),
                    }
                    )?
                    Ok(())
                }
            }
            , buttons: {
                $(
                    $bt_display => |$bt_siv, $bt_ui| -> Result<(), Error> $bt_body
                ),*
            }
            , size: { default }
        }
    };

    ($menu_name:ident
     ,title: { $title:literal }
     ,items: { |$items_ui:pat| -> Result<$items_ok:ty, Error> $items_body:expr }
     ,on_submit: { $($on_submit_display:literal => |$on_submit_siv:pat, $on_submit_item:pat, $on_submit_ui:pat| -> Result<(), Error> $on_submit_body:expr),+ }
     ,on_select: { $($($on_select_display:literal => |$on_select_siv:pat, $on_select_item:pat, $on_select_ui:pat| -> Result<(), Error> $on_select_body:expr),+)? }
     ,buttons: { $($bt_display:literal => |$bt_siv:pat, $bt_ui:pat| -> Result<(), Error> $bt_body:expr),* }
     ,size: { $(($size_x:expr, $size_y:expr))? }
    ) => {
        $crate::menu!{
            $menu_name
            , title: { $title }
            , items: {
                |$items_ui| -> Result<$items_ok, Error> $items_body
            }
            , on_submit: {
                |siv, item, ui| -> Result<(), Error> {
                    match item {
                    $(
                        $on_submit_display => {
                            (|$on_submit_siv: &mut cursive::Cursive,
                              $on_submit_item: &_,
                              $on_submit_ui: &std::rc::Rc<std::cell::RefCell<$crate::app::ui::Ui>>| -> Result<(), $crate::app::ui::Error> { $on_submit_body })(siv, item, &ui)?
                        }
                    )+
                        _ => unreachable!(),
                    }
                    Ok(())
                }
            }
            , on_select: {
                |siv, item, ui| -> Result<(), Error> {
                    $(
                    match item {
                    $(
                        $on_select_display => {
                            (|$on_select_siv: &mut cursive::Cursive,
                              $on_select_item: &_,
                              $on_select_ui: &std::rc::Rc<std::cell::RefCell<$crate::app::ui::Ui>>| -> Result<(), $crate::app::ui::Error> { $on_select_body })(siv, item, &ui)?
                        }
                    )+
                        _ => unreachable!(),
                    }
                    )?
                    Ok(())
                }
            }
            , buttons: {
                $(
                    $bt_display => |$bt_siv, $bt_ui| -> Result<(), Error> $bt_body
                ),*
            }
            , size: { $(($size_x, $size_y))? }
        }
    };

    ($menu_name:ident
     ,title: { $title:literal }
     ,items: { from_submit }
     ,on_submit: { $($on_submit_display:literal => |$on_submit_siv:pat, $on_submit_item:pat, $on_submit_ui:pat| -> Result<(), Error> $on_submit_body:expr),+ }
     ,on_select: { $($($on_select_display:literal => |$on_select_siv:pat, $on_select_item:pat, $on_select_ui:pat| -> Result<(), Error> $on_select_body:expr),+)? }
     ,buttons: { $($bt_display:literal => |$bt_siv:pat, $bt_ui:pat| -> Result<(), Error> $bt_body:expr),* }
     ,size: { default }
    ) => {
        $crate::menu!{
            $menu_name
            , title: { $title }
            , items: {
                |_ui| -> Result<Vec<(String, String)>, Error> { Ok(vec![$(($on_submit_display.to_string(), $on_submit_display.to_string())),+]) }
            }
            , on_submit: {
                |siv, item, ui| -> Result<(), Error> {
                    match item {
                    $(
                        $on_submit_display => {
                            (|$on_submit_siv: &mut cursive::Cursive,
                              $on_submit_item: &_,
                              $on_submit_ui: &std::rc::Rc<std::cell::RefCell<$crate::app::ui::Ui>>| -> Result<(), $crate::app::ui::Error> { $on_submit_body })(siv, item, &ui)?
                        }
                    )+
                        _ => unreachable!(),
                    }
                    Ok(())
                }
            }
            , on_select: {
                |siv, item, ui| -> Result<(), Error> {
                    $(
                    match item {
                    $(
                        $on_select_display => {
                            (|$on_select_siv: &mut cursive::Cursive,
                              $on_select_item: &_,
                              $on_select_ui: &std::rc::Rc<std::cell::RefCell<$crate::app::ui::Ui>>| -> Result<(), $crate::app::ui::Error> { $on_select_body })(siv, item, &ui)?
                        }
                    )+
                        _ => unreachable!(),
                    }
                    )?
                    Ok(())
                }
            }
            , buttons: {
                $(
                    $bt_display => |$bt_siv, $bt_ui| -> Result<(), Error> $bt_body
                ),*
            }
            , size: { default }
        }
    };

    ($menu_name:ident
     ,title: { $title:literal }
     ,items: { from_submit }
     ,on_submit: { $($on_submit_display:literal => |$on_submit_siv:pat, $on_submit_item:pat, $on_submit_ui:pat| -> Result<(), Error> $on_submit_body:expr),+ }
     ,on_select: { $($($on_select_display:literal => |$on_select_siv:pat, $on_select_item:pat, $on_select_ui:pat| -> Result<(), Error> $on_select_body:expr),+)? }
     ,buttons: { $($bt_display:literal => |$bt_siv:pat, $bt_ui:pat| -> Result<(), Error> $bt_body:expr),* }
     ,size: { $(($size_x:expr, $size_y:expr))? }
    ) => {
        $crate::menu!{
            $menu_name
            , title: { $title }
            , items: {
                |_ui| -> Result<Vec<(String, String)>, Error> { Ok(vec![$(($on_submit_display.to_string(), $on_submit_display.to_string())),+]) }
            }
            , on_submit: {
                |siv, item, ui| -> Result<(), Error> {
                    match item {
                    $(
                        $on_submit_display => {
                            (|$on_submit_siv: &mut cursive::Cursive,
                              $on_submit_item: &_,
                              $on_submit_ui: &std::rc::Rc<std::cell::RefCell<$crate::app::ui::Ui>>| -> Result<(), $crate::app::ui::Error> { $on_submit_body })(siv, item, &ui)?
                        }
                    )+
                        _ => unreachable!(),
                    }
                    Ok(())
                }
            }
            , on_select: {
                |siv, item, ui| -> Result<(), Error> {
                    $(
                    match item {
                    $(
                        $on_select_display => {
                            (|$on_select_siv: &mut cursive::Cursive,
                              $on_select_item: &_,
                              $on_select_ui: &std::rc::Rc<std::cell::RefCell<$crate::app::ui::Ui>>| -> Result<(), $crate::app::ui::Error> { $on_select_body })(siv, item, &ui)?
                        }
                    )+
                        _ => unreachable!(),
                    }
                    )?
                    Ok(())
                }
            }
            , buttons: {
                $(
                    $bt_display => |$bt_siv, $bt_ui| -> Result<(), Error> $bt_body
                ),*
            }
            , size: { $(($size_x, $size_y))? }
        }
    };

    ($menu_name:ident
     ,title: {$title:literal}
     ,items: { |$items_ui:pat| -> Result<$items_ok:ty, Error> $items_body:expr }
     ,on_submit: { $(|$on_submit_siv:pat, $on_submit_item:pat, $on_submit_ui:pat| -> Result<(), Error> $on_submit_body:expr)? }
     ,on_select: { $(|$on_select_siv:pat, $on_select_item:pat, $on_select_ui:pat| -> Result<(), Error> $on_select_body:expr)? }
     ,buttons: { $($bt_display:literal => |$bt_siv:pat, $bt_ui:pat| -> Result<(), Error> $bt_body:expr),* }
     ,size: { default }
    ) => {
        $crate::menu!{
            @impl
            $menu_name
            , title: { $title }
            , items: {
                |$items_ui: &std::rc::Rc<std::cell::RefCell<$crate::app::ui::Ui>>| -> Result<$items_ok, $crate::app::ui::Error> {$items_body}
            }
            , on_submit: {
                $(
                    |$on_submit_siv: &mut cursive::Cursive,
                     $on_submit_item: &_,
                     $on_submit_ui: &std::rc::Rc<std::cell::RefCell<$crate::app::ui::Ui>>| -> Result<(), $crate::app::ui::Error> {$on_submit_body}
                )?
            }
            , on_select: {
                $(
                    |$on_select_siv: &mut cursive::Cursive,
                     $on_select_item: &_,
                     $on_select_ui: &std::rc::Rc<std::cell::RefCell<$crate::app::ui::Ui>>| -> Result<(), $crate::app::ui::Error> {$on_select_body}
                )?
            }
            , buttons: {
                $(
                    $bt_display => |$bt_siv: &mut cursive::Cursive,
                                    $bt_ui: &std::rc::Rc<std::cell::RefCell<$crate::app::ui::Ui>>| -> Result<(), $crate::app::ui::Error> {$bt_body}
                ),*
            }
            , size: { (30, 15) }
        }
    };

    ($menu_name:ident
     ,title: {$title:literal}
     ,items: { |$items_ui:pat| -> Result<$items_ok:ty, Error> $items_body:expr }
     ,on_submit: { $(|$on_submit_siv:pat, $on_submit_item:pat, $on_submit_ui:pat| -> Result<(), Error> $on_submit_body:expr)? }
     ,on_select: { $(|$on_select_siv:pat, $on_select_item:pat, $on_select_ui:pat| -> Result<(), Error> $on_select_body:expr)? }
     ,buttons: { $($bt_display:literal => |$bt_siv:pat, $bt_ui:pat| -> Result<(), Error> $bt_body:expr),* }
     ,size: { $(($size_x:expr, $size_y:expr))? }
    ) => {
        $crate::menu!{
            @impl
            $menu_name
            , title: { $title }
            , items: {
                |$items_ui: &std::rc::Rc<std::cell::RefCell<$crate::app::ui::Ui>>| -> Result<$items_ok, $crate::app::ui::Error> {$items_body}
            }
            , on_submit: {
                $(
                    |$on_submit_siv: &mut cursive::Cursive,
                     $on_submit_item: &_,
                     $on_submit_ui: &std::rc::Rc<std::cell::RefCell<$crate::app::ui::Ui>>| -> Result<(), $crate::app::ui::Error> {$on_submit_body}
                )?
            }
            , on_select: {
                $(
                    |$on_select_siv: &mut cursive::Cursive,
                     $on_select_item: &_,
                     $on_select_ui: &std::rc::Rc<std::cell::RefCell<$crate::app::ui::Ui>>| -> Result<(), $crate::app::ui::Error> {$on_select_body}
                )?
            }
            , buttons: {
                $(
                    $bt_display => |$bt_siv: &mut cursive::Cursive,
                                    $bt_ui: &std::rc::Rc<std::cell::RefCell<$crate::app::ui::Ui>>| -> Result<(), $crate::app::ui::Error> {$bt_body}
                ),*
            }
            , size: { $(($size_x, $size_y))? }
        }
    };

    (@impl
     $menu_name:ident
     ,title: { $title:literal }
     ,items: { $items_expr:expr }
     ,on_submit: { $($on_submit_expr:expr)? }
     ,on_select: { $($on_select_expr:expr)? }
     ,buttons: { $($bt_display:literal => $bt_expr:expr),* }
     ,size: { $(($size_x:expr, $size_y:expr))? }
    ) => {
        pub struct $menu_name;

        impl $menu_name {
            #[allow(dead_code)]
            fn name() -> &'static str {
                stringify!($menu_name)
            }

            #[allow(dead_code, unused, unreachable_patterns, clippy::redundant_closure_call)]
            fn make(ui: &std::rc::Rc<std::cell::RefCell<Ui>>) -> Result<impl cursive::View, $crate::app::ui::Error> {
                use cursive::view::{Nameable, Scrollable};

                use cursive_aligned_view::Alignable;

                let select = {
                    let mut s = cursive::views::SelectView::new();
                    $crate::menu!(@with_items s ui $items_expr);
                    $(
                    $crate::menu!(@on_submit s ui $on_submit_expr);
                    )?
                    $(
                    $crate::menu!(@on_select s ui $on_select_expr);
                    )?

                    s.h_align(cursive::align::HAlign::Center)
                        .scrollable()
                        .scroll_x(true)
                        .scroll_y(true)
                        .align_top_center()
                };

                let dialog = {
                    let mut d = cursive::views::Dialog::around(select).h_align(cursive::align::HAlign::Center);
                    $crate::menu!(@title d $title);
                    $(
                    $crate::menu!(@add_button d ui $bt_display $bt_expr);
                    )*
                    $(
                    let d = $crate::menu!(@max_size d ($size_x, $size_y));
                    )?

                    d.with_name($menu_name::name())
                };

                Ok(dialog)
            }
        }
    };

    (@title $view:ident $title:literal) => {
        $view.set_title($title);
    };

    (@with_items $view:ident $ui:ident $expr:expr) => {
        {
            let ui: std::rc::Rc<std::cell::RefCell<$crate::app::ui::Ui>> = std::rc::Rc::clone($ui);
            let items = ($expr)(&ui)?;
            $view.add_all(items)
        }
    };

    (@on_submit $view:ident $ui:ident $expr:expr) => {
        $view.set_on_submit({
            let ui: std::rc::Rc<std::cell::RefCell<$crate::app::ui::Ui>> = std::rc::Rc::clone($ui);
            move |siv: &mut cursive::Cursive, item: &_| {
                let f = ($expr)(siv, item, &ui);

                f.map_err(|err| {
                    log::error!("{}", err);
                    err
                })
                .map_err(|err| {
                    siv.add_layer($crate::app::ui::ErrorView::make(&err));
                    err
                })
                .ok();
            }
        });
    };

    (@on_select $view:ident $ui:ident $expr:expr) => {
        $view.set_on_select({
            let ui: std::rc::Rc<std::cell::RefCell<$crate::app::ui::Ui>> = std::rc::Rc::clone($ui);
            move |siv: &mut cursive::Cursive, item: &_| {
                let f = ($expr)(siv, item, &ui);

                f.map_err(|err| {
                    log::error!("{}", err);
                    err
                })
                .map_err(|err| {
                    siv.add_layer($crate::app::ui::ErrorView::make(&err));
                    err
                })
                .ok();
            }
        });
    };

    (@add_button $view:ident $ui:ident $label:literal $expr:expr) => {
        $view.add_button($label, {
            let ui: std::rc::Rc<std::cell::RefCell<$crate::app::ui::Ui>> = std::rc::Rc::clone($ui);
            move |siv: &mut cursive::Cursive|{
                let f = ($expr)(siv, &ui);

                f.map_err(|err| {
                    log::error!("{}", err);
                    err
                })
                .or_else(|err| {
                   siv.add_layer($crate::app::ui::ErrorView::make(&err));
                   Err(err)
                })
                .ok();
            }
        });
    };

    (@max_size $view:ident ($size_x:expr, $size_y:expr)) => {{
        use cursive::view::Resizable;
        $view.max_size(($size_x, $size_y))
    }};
}
