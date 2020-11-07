use anyhow::Result;
use fs_extra::dir::{CopyOptions, DirOptions};
use std::path::PathBuf;

const SHARE_DIR: &str = "../share";

fn main() -> Result<()> {
    let out_dir = PathBuf::from("..").join("target").join("share");
    if !out_dir.exists() {
        std::fs::create_dir_all(&out_dir)?;
    }

    let targets = {
        let opts = {
            let mut opts = DirOptions::new();
            opts.depth = 1;
            opts
        };
        let mut content = fs_extra::dir::get_dir_content2(SHARE_DIR, &opts)?;
        content.directories = content
            .directories
            .into_iter()
            .filter(|s| !s.ends_with("share"))
            .collect();

        let mut targets: Vec<String> = Vec::new();
        targets.append(&mut content.files);
        targets.append(&mut content.directories);
        targets
    };

    // Copy all `asis` content
    {
        let mut opts = CopyOptions::new();
        opts.overwrite = true;
        fs_extra::copy_items(&targets, &out_dir, &opts)?;
    }

    Ok(())
}
