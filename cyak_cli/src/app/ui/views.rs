use super::Error;
use super::ErrorView;
use super::Ui;

use cursive::views::{Dialog, SelectView, TextView};
use cursive::{Cursive, View};

use std::cell::RefCell;
use std::fs;
use std::ops::Deref;
use std::path::PathBuf;
use std::rc::Rc;

crate::menu! {
    MainMenu,
    title: { "Main" },
    items: { from_submit },
    on_submit: {
        "Select preset" => |siv, _item, ui| -> Result<(), Error> {
            siv.add_layer(SelectPresetMenu::make(ui)?);
            Ok(())
        },
        "Exit" => |_, _, _| -> Result<(), Error> {
            std::process::exit(0);
            Ok(())
        }
    },
    on_select: {},
    buttons: {}
}

fn get_presets(ui: &Rc<RefCell<Ui>>) -> Result<Vec<(String, String)>, Error> {
    let ui = ui.deref().borrow();
    let preset_dir = cyak_core::utils::format_presets_dir(&ui.share_dir);
    let paths = fs::read_dir(&preset_dir).map_err(|e| Error::Fatal(format!("{}", e)))?;

    let mut res = Vec::new();
    for p in paths.into_iter() {
        let p = p.map_err(|e| Error::Fatal(e.to_string()))?;
        let preset_path = p
            .path()
            .into_os_string()
            .into_string()
            .map_err(|e| Error::Fatal(format!("{:?}", e)))?;

        let preset_config = cyak_core::load_preset_config(&preset_path)
            .map_err(|e| Error::Fatal(format!("\"{}\": {}", preset_path, e)))?;
        let preset_name = if preset_config.name == ui.preset_config.name {
            format!("> {} <", preset_config.name)
        } else {
            preset_config.name
        };
        res.push((preset_name, preset_path));
    }

    res.push(("Back".to_string(), "Back".to_string()));
    Ok(res)
}

crate::menu! {
    SelectPresetMenu,
    title: { "Main" },
    items: {
        |ui| -> Result<Vec<(String, String)>, Error> {
            get_presets(ui)
        }
    },
    on_submit: {
        "Back" => |siv, _item, _ui| -> Result<(), Error> {
            siv.pop_layer();
            Ok(())
        },
        other => |siv, _item, ui| -> Result<(), Error> {
            {
                let mut ui_mut = ui.deref().borrow_mut();
                let preset_dir = PathBuf::from(other);
                ui_mut.preset_config = cyak_core::load_preset_config(&preset_dir).map_err(|e| Error::Regular(format!("{}", e)))?;
                ui_mut.ctx.preset_dir = preset_dir;
            }

            let presets = get_presets(&ui)?;
            siv.call_on_name(SelectPresetMenu::name(), move |view: &mut SelectView| {
                view.clear();
                view.add_all(presets);
            });

            Ok(())
        }
    },
    on_select: {},
    buttons: {}
}
