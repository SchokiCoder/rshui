// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023  Andy Frank Schoknecht

use serde::Deserialize;
use std::collections::HashMap;
use std::io::Read;
use std::fs::File;
use std::path::Path;

use crate::color::*;
use crate::menu::*;

const PATH_CFG_ETC: &str = "/etc/hui.toml";
const PATH_CFG_XDG: &str = "~/.config/hui.toml";
const PATH_CFG_DOT: &str = "~/.hui.toml";
const PATH_CFG_LCL: &str = "./hui.toml";

#[derive(Deserialize)]
pub struct CfgKeys {
	pub left: char,
	pub down: char,
	pub up: char,
	pub right: char,
	pub execute: char,
	pub cmdmode: char,
	pub cmdenter: char,
}

#[derive(Deserialize)]
pub struct CfgColors {
	pub header: TextColor,
	pub title: TextColor,
	pub entry: TextColor,
	pub entry_hover: TextColor,
	pub cmdline: TextColor,
	pub feedback: TextColor,
	pub std: TextColor,
}

#[derive(Deserialize)]
pub struct Config {
	pub header: String,
	
	pub entry_menu_prefix: String,
	pub entry_menu_postfix: String,
	pub entry_shell_prefix: String,
	pub entry_shell_postfix: String,
	
	pub keys: CfgKeys,
	
	pub menus: HashMap<String, Menu>,
	
	pub colors: CfgColors,
}

impl Config
{
	pub fn from_file() -> Config
	{
		let mut cfgstr = String::new();
		let cfgpath: &str;
		let mut f: File; 

		if Path::new(PATH_CFG_ETC).exists() {
			cfgpath = PATH_CFG_ETC;
		} else if Path::new(PATH_CFG_XDG).exists() {
			cfgpath = PATH_CFG_XDG;
		} else if Path::new(PATH_CFG_DOT).exists() {
			cfgpath = PATH_CFG_DOT;
		} else if Path::new(PATH_CFG_LCL).exists() {
			cfgpath = PATH_CFG_LCL;
		} else {
			panic!("No config found.");
		}

		f = File::open(cfgpath).unwrap();
		f.read_to_string(&mut cfgstr).unwrap();

		return toml::from_str(cfgstr.as_ref()).unwrap();
	}
}
