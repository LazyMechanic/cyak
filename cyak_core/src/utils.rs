use std::path::{Path, PathBuf};

use crate::consts::*;
use crate::Error;

pub fn format_config_template<P: AsRef<Path>>(preset_dir: P) -> PathBuf {
    preset_dir
        .as_ref()
        .join(TEMPLATES_DIR)
        .join(CONFIG_TEMPLATE_FILE)
}

pub fn format_executable_template<P: AsRef<Path>>(preset_dir: P) -> PathBuf {
    preset_dir
        .as_ref()
        .join(TEMPLATES_DIR)
        .join(EXECUTABLE_TEMPLATE_FILE)
}

pub fn format_interface_template<P: AsRef<Path>>(preset_dir: P) -> PathBuf {
    preset_dir
        .as_ref()
        .join(TEMPLATES_DIR)
        .join(INTERFACE_TEMPLATE_FILE)
}

pub fn format_library_template<P: AsRef<Path>>(preset_dir: P) -> PathBuf {
    preset_dir
        .as_ref()
        .join(TEMPLATES_DIR)
        .join(LIBRARY_TEMPLATE_FILE)
}

pub fn format_project_template<P: AsRef<Path>>(preset_dir: P) -> PathBuf {
    preset_dir
        .as_ref()
        .join(TEMPLATES_DIR)
        .join(PROJECT_TEMPLATE_FILE)
}

pub fn format_test_template<P: AsRef<Path>>(preset_dir: P) -> PathBuf {
    preset_dir
        .as_ref()
        .join(TEMPLATES_DIR)
        .join(TEST_TEMPLATE_FILE)
}

pub fn format_asis_dir<P: AsRef<Path>>(preset_dir: P) -> PathBuf {
    preset_dir.as_ref().join(ASIS_DIR)
}

pub fn format_project_config<P: AsRef<Path>>(project_dir: P) -> PathBuf {
    project_dir
        .as_ref()
        .join(CYAK_CONFIG_DIR)
        .join(CYAK_CONFIG_FILE)
}

pub fn format_preset_config<P: AsRef<Path>>(preset_dir: P) -> PathBuf {
    preset_dir.as_ref().join(PRESET_CONFIG_FILE)
}

pub fn format_cmake_modules_dir<P: AsRef<Path>>(project_dir: P) -> PathBuf {
    project_dir.as_ref().join(CMAKE_MODULES_DIR)
}

pub fn format_lib_config_file<P: AsRef<Path>>(cmake_modules_dir: P, target_name: P) -> PathBuf {
    cmake_modules_dir.as_ref().join(format!(
        "{}-config.cmake.in",
        target_name.as_ref().display()
    ))
}

/// Return `Ok(())` if file exists.
/// Return `Err(Error)` if file not exists or path is not a file.
pub fn check_file_existence<P: AsRef<Path>>(p: P) -> Result<(), Error> {
    if !p.as_ref().exists() {
        return Error::FileNotFound(p.as_ref().to_path_buf()).fail();
    }

    if !p.as_ref().is_file() {
        return Error::NotFile(p.as_ref().to_path_buf()).fail();
    }

    Ok(())
}

/// Return `Ok(())` if dir exists.
/// Return `Err(Error)` if dir not exists or path is not a dir.
pub fn check_dir_existence<P: AsRef<Path>>(p: P) -> Result<(), Error> {
    if !p.as_ref().exists() {
        return Error::DirNotFound(p.as_ref().to_path_buf()).fail();
    }

    if !p.as_ref().is_dir() {
        return Error::NotDir(p.as_ref().to_path_buf()).fail();
    }

    Ok(())
}

/// Return `Ok(())` if dir not exist and create it, or do nothing if already exists.
/// Return `Err(Error)` if create dir ends with err.
pub fn create_nonexistent_dir<P: AsRef<Path>>(p: P) -> Result<(), Error> {
    let p = p.as_ref();
    if !p.exists() {
        std::fs::create_dir(p)?
    }

    Ok(())
}

/// Return `Ok(())` if dir not exists and create all path, or do nothing if already exists.
/// Return `Err(Error)` if create dir ends with err.
pub fn create_nonexistent_dir_all<P: AsRef<Path>>(p: P) -> Result<(), Error> {
    let p = p.as_ref();
    if !p.exists() {
        std::fs::create_dir_all(p)?
    }

    Ok(())
}
