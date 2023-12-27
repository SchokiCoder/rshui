// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023  Andy Frank Schoknecht

use serde::Deserialize;
use std::io::Read;
use std::fs::File;
use std::path::Path;

use crate::color::TextColor;
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
pub struct Config<'a> {
	pub header: &'a str,
	
	pub entry_menu_prefix: &'a str,
	pub entry_menu_postfix: &'a str,
	pub entry_shell_prefix: &'a str,
	pub entry_shell_postfix: &'a str,
	
	pub keys: CfgKeys,
	
	pub menus: Vec<Menu<'a>>,
	
	pub colors: CfgColors,
}

impl<'a> Config<'a>
{
	pub fn from_file() -> Config<'a>
	{
		let cfgstr: String;
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
