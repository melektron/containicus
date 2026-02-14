/*
ELEKTRON © 2026 - now
Written by melektron
www.elektron.work
13.02.26, 21:32
All rights reserved.

This source code is licensed under the Apache-2.0 license found in the
LICENSE file in the root directory of this source tree. 

Managing container projects in NixOS
*/

mod cli_adapter;
mod docker_compose_cli;

use clap::Parser;
use log::{error, info, warn};

/// Managing container projects in NixOS 
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {

    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Trace) // show everything
        .format(|buf, record| {
            use std::io::Write;
            let level = record.level();
            let color = match level {
                log::Level::Error => "\x1b[31m", // red
                log::Level::Warn  => "\x1b[33m", // yellow
                log::Level::Info  => "\x1b[32m", // green
                log::Level::Debug => "\x1b[34m", // blue
                log::Level::Trace => "\x1b[35m", // magenta
            };
            writeln!(buf, "{}[{}] {}\x1b[0m", color, level, record.args())
        })
        .init();


    if let Some(res) = docker_compose_cli::parse_help_text() {
        info!("{res:?}");
    } else {
        error!("Failed to parse help!");
    }


    //let args = Args::parse();
    //
    //for _ in 0..args.count {
    //    println!("Hello {}!", args.name);
    //}
}