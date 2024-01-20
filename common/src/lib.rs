// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023 - 2024  Andy Frank Schoknecht

pub mod color;
pub mod config;

use crate::config::ComCfg;

pub const SIGINT:  char = '\x03';
pub const SIGTSTP: char = '\x04';

pub fn draw_upper(comcfg: &ComCfg, header: &str, title: &str)
{
	print!("{}{}{}{}{}\n",
	       comcfg.colors.header.fg,
	       comcfg.colors.header.bg,
	       header,
	       comcfg.colors.std.fg,
	       comcfg.colors.std.bg);

	print!("{}{}{}{}{}\n",
	       comcfg.colors.title.fg,
	       comcfg.colors.title.bg,
	       title,
	       comcfg.colors.std.fg,
	       comcfg.colors.std.bg);
}
