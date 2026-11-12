// Copyright (C) 2026 CharOfString <root@charofstring.cc>
//
//
// This software is free software: you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This software is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with this software. If
// not, see <https://www.gnu.org/licenses/>.

use std::env;
use std::path::Path;
use std::process::exit;

fn is_config_exist() -> bool {
    let build_conf_path = Path::new("samoyed.conf");
    if build_conf_path.exists() {
        if build_conf_path.is_file() {
            return true;
        }
        return false;
    }
    false
}

fn print_usage() {
    println!("Samoyed Remote Build");
    println!("Usage: samoyed <options>\n");
    println!("Options:");
    println!("\t-h, --help  Show this help message.");
}

fn main() {
    println!("Welcome to Samoyed!");
    print!("Please note that this software is non-production safe ");
    println!("and is intended for local network build only.\n");

    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 0 {
        // If there is other params
        for cur in args {
            match cur.as_str() {
                "-h" | "--help" => print_usage(),
                _ => eprintln!("Warn: Unknown command \"{cur}\". This argument will be omitted.")
            }
        }
    }

    // If there is no params, find the build config
    if !is_config_exist() {
        eprint!("You will need to use parameters to setup Samoyed or provide a ");
        eprint!("valid config file on samoyed.conf!\n");
        eprintln!("If you'd like to use parameters, please run samoyed -h for more info.");
        exit(1);
    }
}
