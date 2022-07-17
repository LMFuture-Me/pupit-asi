// A tool to make it easier to create ADB scripts.
// Author: LMFuture <https://github.com/LMFuture-Me>
// Copyright (c) 2022 LMFuture <https://github.com/LMFuture-Me>

mod adb;
mod command_processor;
mod language;

use command_processor::CommandLine;


fn main() {
    // TODO: make it a multi-language program.
    // TODO: make a ctrl+c handler to make it safer to flash smart phone.
    // TODO: multi devices support


    // Check whether adb is available
    let adb_dir = adb::Exist::new();
    if !adb_dir.exist {
        std::process::exit(120);
    }

    // Get commandline arguments
    let args: Vec<String> = std::env::args().collect();

    // Process commandline arguments
    if args.len() < 2 {
        println!("Too few arguments.");
        std::process::exit(119);
    }

    // Create a commandline object
    let commandline = CommandLine::new(args,adb_dir.path);
    commandline.run();


}
