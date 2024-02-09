// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023 - 2024  Andy Frank Schoknecht

use crate::color::*;

use serde::Deserialize;
use std::io::Read;
use std::fs::File;
use std::path::Path;

const PATH_CFGPOSTFIX_COMMON: &str = "common.toml";
const PATHS_CFG: &'static [&'static str] = &[
	"/etc/hui/",
	"~/.config/hui/",
	"~/.hui/",
	"./",
];

#[derive(Deserialize)]
pub struct CfgKeys {
	pub left: char,
	pub down: char,
	pub up: char,
	pub right: char,
	pub cmdmode: char,
	pub cmdenter: char,
	pub quit: char,
}

#[derive(Deserialize)]
pub struct CfgColors {
	pub header: TextColor,
	pub title: TextColor,
	pub cmdline: TextColor,
	pub feedback: TextColor,
	pub std: TextColor,
}

#[derive(Deserialize)]
pub struct ComCfg {	
	pub keys: CfgKeys,
	
	pub colors: CfgColors,
}

impl ComCfg
{
	pub fn from_file() -> ComCfg
	{
		let res = toml::from_str(&read_cfg_file(PATH_CFGPOSTFIX_COMMON));
		
		match res {
		Ok(ret) => {
			return ret;
		}

		Err(e) => {
			panic!("Config \"{}\" could not be parsed: {}",
			       PATH_CFGPOSTFIX_COMMON,
			       e);
		}}
	}
}

pub fn find_cfg_path(path_postfix: &str) -> Option<String>
{
	for item in PATHS_CFG {
		let pstr = format!("{}{}", *item, path_postfix);
		let p = Path::new(&pstr);
		if p.exists() {
			return Some(pstr);
		}
	}
	
	return None;
}

pub fn read_cfg_file(path_postfix: &str) -> String
{
	let mut cfgstr = String::new();
	let mut f: File; 

	let path = find_cfg_path(path_postfix);
	if path == None {
		panic!("Config \"{}\" not found.", path_postfix);
	}

	let path_str = path.unwrap();
	f = File::open(path_str)
	         .expect(format!("Config \"{}\" could not be opened.",
	                         path_postfix).as_ref());

	f.read_to_string(&mut cfgstr)
	 .expect(format!("Config \"{}\" could not be read.", path_postfix).as_ref());
	
	return cfgstr;
}
