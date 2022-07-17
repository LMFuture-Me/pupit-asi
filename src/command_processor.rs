use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use crate::adb;
use crate::language;

#[derive(Debug)]
pub struct CommandLine {
    args: Vec<String>,
    adb_dir: String,
    language: language::Language,
}

#[derive(Debug)]
pub struct Device {
    id: String,
    authorized: bool,
}

impl CommandLine {
    // Create a new commandline processor
    pub fn new(args: Vec<String>, adb_dir: String) -> Self {
        Self {
            args,
            adb_dir,
            language: language::Language::new(),
        }
    }

    // Handle wrong arguments
    fn err_handler(&self) {
        println!("Error: Unknown action: {}", self.args[1]);
        std::process::exit(201);
    }

    // Fastboot flash images
    // TODO: implement dtb base kernel-offset tags-offset
    fn adb_flash_image(args: Vec<String>) {}

    // Fastboot devices

    // Fastboot reboot

    // Fastboot wipe user data

    // TODO: implement force

    // Fastboot boot image

    // Fastboot erase specified partition

    // Fastboot set active slot

    // TODO: Fastboot wipe super

    // Fetch partition image

    // TODO: update ZIP

    // TODO: Flash all


    // ADB install apk.

    // ADB kill server

    // ADB devices
    pub fn adb_devices(&self) -> Vec<Device> {
        let process = Command::new(format!("{}adb", self.adb_dir))
            .arg("devices")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let mut readbuf = BufReader::new(process.stdout.unwrap());
        let itrator = readbuf.lines();
        let mut counter = 0;
        let mut stat: bool = false;
        let mut devices_id: Vec<Device> = Vec::new();
        for line in itrator {
            let mut line = line.unwrap();
            if stat == true {
                counter = counter + 1;
                if line.ends_with("device") {
                    let mut line = line.trim();
                    let length = line.as_bytes().len();
                    let mut line = line.to_string();
                    line.truncate(length - 7);
                    devices_id.push(Device {
                        id: line,
                        authorized: true,
                    });
                }
                if line.ends_with("unauthorized") {
                    let mut line = line.trim();
                    let length = line.as_bytes().len();
                    let mut line = line.to_string();
                    line.truncate(length - 13);
                    devices_id.push(Device {
                        id: line,
                        authorized: false,
                    });
                }
            }
            if line.starts_with("List of devices attached") {
                stat = true;
            }
        }
        let counter = counter - 1;
        println!("Device count: {}", counter);
        println!("Device ID: {:#?}", devices_id);
        return devices_id;
    }

    // ADB Push file

    // ADB pull file

    // ADB side-load

    // ADB reboot

    // ADB detach

    // TODO: ADB attach


    // Install ADB Toolchain
    fn install_adb(&self) {}

    // Process it and do it.
    pub fn run(&self) {
        // Debug
        println!("{:#?}", self.args);

        match self.args[1].to_lowercase().as_str() {
            "adb" => match self.args[2].to_lowercase().as_str() {
                "devices" => self.adb_devices(),
                _ => todo!(),
            }
            "fastboot" => match self.args[2].to_lowercase().as_str() {
                _ => todo!(),
            }
            _ => todo!(),
        };
    }
}

