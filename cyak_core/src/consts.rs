pub const ASIS_DIR: &str = "asis";

pub const PRESET_CONFIG_FILE: &str = "config.yaml";
pub const TEMPLATES_DIR: &str = "templates";
pub const CONFIG_TEMPLATE_FILE: &str = "config.hbs";
pub const EXECUTABLE_TEMPLATE_FILE: &str = "executable.hbs";
pub const INTERFACE_TEMPLATE_FILE: &str = "interface.hbs";
pub const LIBRARY_TEMPLATE_FILE: &str = "library.hbs";
pub const PROJECT_TEMPLATE_FILE: &str = "project.hbs";
pub const TEST_TEMPLATE_FILE: &str = "test.hbs";
pub const PRESETS_DIR: &str = "presets";
pub const LICENSES_DIR: &str = "licenses";

pub const CMAKE_MODULES_DIR: &str = "cmake";
pub const TESTS_DIR: &str = "tests";
pub const SOURCE_DIR: &str = "src";
pub const INTERFACE_DIR: &str = "include";
pub const CMAKE_FILE: &str = "CMakeLists.txt";
pub const LICENSE_FILE: &str = "LICENSE";

pub const EXEC_SRC_FILE: &str = "main.cpp";
pub const EXEC_SRC: &str = r##"
#include <iostream>

int main() {
    std::cout << "Hello, world" << std::endl;
    return 0;
}
"##;

pub const LIB_SRC: &str = r##"
#pragma once
#include <iostream>

void print_hello_world() {
    std::cout << "Hello, world" << std::endl;
}
"##;
