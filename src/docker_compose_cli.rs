/*
ELEKTRON © 2026 - now
Written by melektron
www.elektron.work
13.02.26, 21:26
All rights reserved.

This source code is licensed under the Apache-2.0 license found in the
LICENSE file in the root directory of this source tree. 

Support for interacting with and providing a docker-compose-like CLI.
*/

use std::{default, process::Command};
use log::{info, warn, debug};
use regex_macro::regex;


#[derive(Debug, Default)]
pub struct ComposeHelp {
    usage: String,
    description: String,
    sections: Vec<Section>,
}

#[derive(Debug)]
pub enum Section {
    Options(String, Vec<OptionEntry>),
    Commands(String, Vec<CommandEntry>),
}

#[derive(Debug)]
pub struct OptionEntry {
    short: Option<String>,
    long: Option<String>,
    value: Option<String>,
    description: String,
}

#[derive(Debug)]
pub struct CommandEntry {
    name: String,
    description: String,
}



pub fn get_help_text() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("docker")
        .args(["compose", "--help"])
        .output()?;

    Ok(String::from_utf8(output.stdout)?)
}

pub fn parse_help_text() -> Option<ComposeHelp> {
    let Ok(text) = get_help_text() else { return None };

    enum LastLineKind {
        None,
        Usage,
        Description,
        SectionHeader,
        Option,
        Command,
    }
    let mut last_line_kind = LastLineKind::None;
    let mut help = ComposeHelp::default();
    
    for line in text.lines() {
        //if line.is_empty() { continue };

        // usage line
        if let Some(usage) = line.strip_prefix("Usage: ") {
            help.usage = usage.to_owned();
            last_line_kind = LastLineKind::Usage;
            continue;
        }

        // beginning of a new section
        if let Some(section_header) = line.strip_suffix(":") {
            if line.contains("Option") {
                help.sections.push(Section::Options(section_header.to_owned(), vec![]));
            } else {
                help.sections.push(Section::Commands(section_header.to_owned(), vec![]));
            }
            last_line_kind = LastLineKind::SectionHeader;
            continue;
        }
        
        // command
        let cmd_re = regex!(r"^\s{2}([a-z0-9_-]+)\s{2,}(.*)$");
        if let Some(caps) = cmd_re.captures(line) {
            let Some(Section::Commands(_, cmds)) = help.sections.last_mut() else {
                warn!("Stray command in docker compose help text, ignoring: {line}");
                continue;
            };
            cmds.push(CommandEntry { 
                name: caps[1].to_owned(), 
                description: caps[2].to_owned()
            });
            last_line_kind = LastLineKind::Command;
            continue;
        }

        // option
        let opt_re = regex!(r"^\s{2}(?:(-\w),)?\s+(--[a-z0-9-]+)(?:\s+([A-Za-z0-9]+))?\s{2,}(.*)$");
        if let Some(caps) = opt_re.captures(line) {
            let Some(Section::Options(_, opts)) = help.sections.last_mut() else {
                warn!("Stray option in docker compose help text, ignoring: {line}");
                continue;
            };
            opts.push(OptionEntry { 
                short: caps.get(1).map(|x| x.as_str().to_owned()),
                long: caps.get(2).map(|x| x.as_str().to_owned()),
                value: caps.get(3).map(|x| x.as_str().to_owned()),
                description: caps.get(4).expect("Missing description for docker compose subcommand").as_str().to_owned(),
            });
            last_line_kind = LastLineKind::Option;
            continue;
        }
        
        // if it was not a command or option but it matches the following,
        // it is likely a second description line of the previous element
        if matches!(last_line_kind, LastLineKind::Command | LastLineKind::Option)
           && let Some(caps) = regex!(r"^\s{3,}(.*)$").captures(line) {

            match help.sections.last_mut() {
                Some(Section::Options(_, opts)) => {
                    if let Some(opt) = opts.last_mut() {
                        opt.description.push_str(&caps[1]);
                        continue;
                    }
                }
                Some(Section::Commands(_, cmds)) => {
                    if let Some(cmd) = cmds.last_mut() {
                        cmd.description.push_str(&caps[1]);
                        continue;
                    }
                }
                None => {}
            }

            warn!("Stray description extension in docker compose help text, ignoring: {line}");
            continue;
        }

        // if none of the above cases matched, and we are after Usage or description, the text is a description
        if matches!(last_line_kind, LastLineKind::Description | LastLineKind::Usage) {
            help.description.push_str(line);
            help.description.push_str("\n");
            last_line_kind = LastLineKind::Description;
            continue;
        }

        // in all other cases, we simply ignore the line:
        debug!("Ignoring line: {line}");
    }

    help.description.trim();

    Some(help)
}