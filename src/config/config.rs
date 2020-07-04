use chrono::prelude::*;
use serde::{Deserialize, Serialize};

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
            version:             "0.1.0".to_string(),
            template_name:       "".to_string(),
            project_template:    "".to_string(),
            executable_template: "".to_string(),
            interface_template:  "".to_string(),
            library_template:    "".to_string(),
            lib_config_template: "".to_string(),
            test_template:       "".to_string(),
        }
    }

    pub fn with_version<T: Into<String>>(mut self, v: T) -> Self {
        self.version = v.into();
        self
    }

    pub fn with_template_name<T: Into<String>>(mut self, v: T) -> Self {
        self.template_name = v.into();
        self
    }

    pub fn with_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project_template = v.into();
        self
    }

    pub fn with_executable<T: Into<String>>(mut self, v: T) -> Self {
        self.executable_template = v.into();
        self
    }

    pub fn with_interface<T: Into<String>>(mut self, v: T) -> Self {
        self.interface_template = v.into();
        self
    }

    pub fn with_library<T: Into<String>>(mut self, v: T) -> Self {
        self.library_template = v.into();
        self
    }

    pub fn with_lib_config<T: Into<String>>(mut self, v: T) -> Self {
        self.lib_config_template = v.into();
        self
    }

    pub fn with_test<T: Into<String>>(mut self, v: T) -> Self {
        self.test_template = v.into();
        self
    }
}
