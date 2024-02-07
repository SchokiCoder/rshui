// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023 - 2024  Andy Frank Schoknecht

use common::color::TextColor;
use common::config::{read_cfg_file};
use serde::Deserialize;

const PATH_CFGPOSTFIX_COU: &str = "courier.toml";

#[derive(Deserialize)]
pub struct CfgColors {
	pub content: TextColor,
}

#[derive(Deserialize)]
pub struct CouCfg {
	pub header: String,

	pub colors: CfgColors,
}

impl CouCfg
{
	pub fn from_file() -> CouCfg
	{
		let res = toml::from_str(&read_cfg_file(PATH_CFGPOSTFIX_COU));
		
		match res {
		Ok(ret) => {
			return ret;
		}

		Err(e) => {
			panic!("Config \"{}\" could not be parsed: {}",
			       PATH_CFGPOSTFIX_COU,
			       e);
		}}
	}
}
