// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023 - 2024  Andy Frank Schoknecht

use crate::menu::*;
use common::color::TextColor;
use common::config::{read_cfg_file};

use serde::Deserialize;
use std::collections::HashMap;

const PATH_CFGPOSTFIX_HUI: &str = "hui.toml";

#[derive(Deserialize)]
pub struct CfgKeys {
	pub execute: char,
}

#[derive(Deserialize)]
pub struct CfgColors {
	pub entry: TextColor,
	pub entry_hover: TextColor,
}

#[derive(Deserialize)]
pub struct HuiCfg {
	pub header: String,
	
	pub entry_menu_prefix: String,
	pub entry_menu_postfix: String,
	pub entry_shell_prefix: String,
	pub entry_shell_postfix: String,
	pub entry_shellsession_prefix: String,
	pub entry_shellsession_postfix: String,
	
	pub keys: CfgKeys,
	
	pub menus: HashMap<String, Menu>,
	
	pub colors: CfgColors,
}

impl HuiCfg
{
	pub fn from_file() -> HuiCfg
	{
		let res = toml::from_str(&read_cfg_file(PATH_CFGPOSTFIX_HUI));
		
		match res {
		Ok(ret) => {
			return ret;
		}

		Err(e) => {
			panic!("Config \"{}\" could not be parsed: {}",
			       PATH_CFGPOSTFIX_HUI,
			       e);
		}}
	}
}
