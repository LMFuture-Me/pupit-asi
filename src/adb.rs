use std::{env};
use std::process::Command;

#[derive(Debug)]
pub struct Exist {
    pub exist: bool,
    system_env: bool,
    pub path: String,
}

impl Exist {
    // get current executable path without executable name
    pub fn get_path() -> String {
        let mut path = String::new();
        let mut path_buf = env::current_exe().unwrap();
        path_buf.pop();
        path_buf.push("");
        path = path_buf.to_str().unwrap().to_string();
        path
    }

    // get current executable path
    fn get_path_with_executable() -> String {
        let mut path = String::new();
        let mut path_buf = env::current_exe().unwrap();
        path = path_buf.to_str().unwrap().to_string();
        path
    }

    // Check if system has adb installed
    fn check_if_env_has_adb() -> bool {
        let commandline = Command::new("adb").output();
        match commandline {
            Ok(_commandline) => true,
            Err(_err) => false,
        }
    }

    // Check if current directory has adb tool.
    fn check_if_current_directory_has_adb() -> Result<String, ()> {
        let path = Self::get_path();
        let commandline = Command::new(format!("{}adb", path).as_str()).output();
        match commandline {
            Ok(_commandline) => Ok(path),
            Err(_err) => Err(()),
        }
    }

    // Check if specified directory has adb tool.
    fn check_if_specified_directory_has_adb() -> Result<String, ()> {
        let path = match env::var("ASI_SPECIFIC_ADB_DIR") {
            Ok(val) => val,
            Err(_) => return Err(()),
        };
        let commandline = Command::new(format!("{}adb", path)).output();
        match commandline {
            Ok(_commandline) => Ok(path),
            Err(_err) => Err(()),
        }
    }

    // Create a new adb checker struct.
    pub fn new() -> Self {
        match Self::check_if_specified_directory_has_adb() {
            Ok(val) => return Exist {
                exist: true,
                system_env: false,
                path: val.to_string(),
            },
            Err(_err) => {}
        };

        match Self::check_if_current_directory_has_adb() {
            Ok(val) => return Exist {
                exist: true,
                system_env: false,
                path: val.to_string(),
            },
            Err(_err) => {}
        };

        if Self::check_if_env_has_adb() {
            return Exist {
                exist: true,
                system_env: true,
                path: String::from(""),
            };
        };

        Exist {
            exist: false,
            system_env: false,
            path: String::from(""),
        }
    }
}