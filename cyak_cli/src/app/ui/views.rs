use super::Error;

use std::fs;
use std::ops::Deref;
use std::path::PathBuf;

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
    buttons: {},
    size: { default }
}

crate::menu! {
    SelectPresetMenu,
    title: { "Main" },
    items: {
        |ui| -> Result<Vec<(String, String)>, Error> {
            let ui_mut = ui.deref().borrow();
            let preset_dir = cyak_core::utils::format_presets_dir(&ui_mut.share_dir);
            let paths = fs::read_dir(&preset_dir).map_err(|e| Error::Fatal(e.to_string()))?;

            let mut res = Vec::new();
            for p in paths.into_iter() {
                let p = p.map_err(|e| Error::Fatal(e.to_string()))?;
                let full_path = p.path().into_os_string().into_string().map_err(|e| Error::Fatal(format!("{:?}", e)))?;
                let filename = p.file_name().into_string().map_err(|e| Error::Fatal(format!("{:?}", e)))?;
                res.push((filename, full_path));
            }

            res.push(("Back".to_string(), "Back".to_string()));
            Ok(res)
        }
    },
    on_submit: {
        |siv, item, ui| -> Result<(), Error> {
            let mut ui_mut = ui.deref().borrow_mut();
            match item {
                "Back" => {
                    siv.pop_layer();
                }
                other => {
                    let preset_dir = PathBuf::from(other);
                    ui_mut.preset_config = cyak_core::load_preset_config(&preset_dir).map_err(|e| Error::Regular(format!("{}", e)))?;
                    ui_mut.ctx.preset_dir = preset_dir;
                }
            };
            Ok(())
        }
    },
    on_select: {},
    buttons: {},
    size: { default }
}
