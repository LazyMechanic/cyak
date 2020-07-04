use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use super::consts::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    date:                DateTime<Utc>,
    version:             String,
    template_name:       String,
    project_template:    String,
    executable_template: String,
    interface_template:  String,
    library_template:    String,
    lib_config_template: String,
    test_template:       String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            date:                Utc::now(),
            version:             PRG_VERSION.to_string(),
            template_name:       "".to_string(),
            project_template:    "".to_string(),
            executable_template: "".to_string(),
            interface_template:  "".to_string(),
            library_template:    "".to_string(),
            lib_config_template: "".to_string(),
            test_template:       "".to_string(),
        }
    }

    pub fn with_template_name<T: AsRef<str>>(mut self, name: T) -> Self {
        self.template_name = name;
        self
    }

    pub fn with_project<T: AsRef<str>>(mut self, v: T) -> Self {
        self.project_template = v;
        self
    }

    pub fn with_executable<T: AsRef<str>>(mut self, v: T) -> Self {
        self.executable_template = v;
        self
    }

    pub fn with_interface<T: AsRef<str>>(mut self, v: T) -> Self {
        self.interface_template = v;
        self
    }

    pub fn with_library<T: AsRef<str>>(mut self, v: T) -> Self {
        self.library_template = v;
        self
    }

    pub fn with_lib_config<T: AsRef<str>>(mut self, v: T) -> Self {
        self.lib_config_template = v;
        self
    }

    pub fn with_test<T: AsRef<str>>(mut self, v: T) -> Self {
        self.test_template = v;
        self
    }
}
