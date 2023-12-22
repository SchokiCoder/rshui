// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023  Andy Frank Schoknecht

use crate::color::*;
use crate::menu::*;

pub const HEADER: &str = "Example config\n";

pub const MENU_TESTCHAMBER: Menu = Menu {
	title: "Test chamber\n\
		------------",
	entries: &[
		Entry {
			caption: "Start the rotors",
			content: EntryContent::Shell("echo \"... very good\""),
		},
	]
};

pub const MENU_MAIN: Menu = Menu {
	title: "Main Menu\n\
		---------",
	entries: &[
		Entry {
			caption: "Test chamber",
			content: EntryContent::Menu(&MENU_TESTCHAMBER)
		},

		Entry {
			caption: "Show current user",
			content: EntryContent::Shell("echo \"$USER\""),
		},

		Entry {
			caption: "Rust Dummy",
			content: EntryContent::Rust,
		},

		Entry {
			caption: "Clear ~/temp",
			content: EntryContent::Shell("echo \"\" > ~/temp"),
		},
	],
};

pub const ET_MENU_PREFIX:  &str = "> [";
pub const ET_MENU_POSTFIX: &str = "]";
pub const ET_RS_PREFIX:    &str = "> ";
pub const ET_RS_POSTFIX:   &str = "!";
pub const ET_SH_PREFIX:    &str = "> ";
pub const ET_SH_POSTFIX:   &str = "";

pub const HEADER_FG: FgColor = FgColor {
	active: false,
	r: 255,
	g: 255,
	b: 255
};

pub const HEADER_BG: BgColor = BgColor {
	active: false,
	r: 30,
	g: 30,
	b: 30
};

pub const TITLE_FG: FgColor = FgColor {
	active: false,
	r: 255,
	g: 255,
	b: 255
};

pub const TITLE_BG: BgColor = BgColor {
	active: false,
	r: 30,
	g: 30,
	b: 30
};

pub const ENTRY_FG: FgColor = FgColor {
	active: false,
	r: 255,
	g: 255,
	b: 255
};

pub const ENTRY_BG: BgColor = BgColor {
	active: false,
	r: 30,
	g: 30,
	b: 30
};

pub const ENTRY_HOVER_FG: FgColor = FgColor {
	active: true,
	r: 0,
	g: 0,
	b: 0
};

pub const ENTRY_HOVER_BG: BgColor = BgColor {
	active: true,
	r: 255,
	g: 255,
	b: 255
};

pub const CMDLINE_FG: FgColor = FgColor {
	active: false,
	r: 255,
	g: 255,
	b: 255
};

pub const CMDLINE_BG: BgColor = BgColor {
	active: false,
	r: 30,
	g: 30,
	b: 30
};

pub const FEEDBACK_FG: FgColor = FgColor {
	active: true,
	r: 160,
	g: 160,
	b: 160
};

pub const FEEDBACK_BG: BgColor = BgColor {
	active: false,
	r: 30,
	g: 30,
	b: 30
};

pub const DEFAULT_FG: FgColor = FgColor {
	active: false,
	r: 255,
	g: 255,
	b: 255
};

pub const DEFAULT_BG: BgColor = BgColor {
	active: false,
	r: 0,
	g: 0,
	b: 0
};

pub const KEY_LEFT: char = 'h';
pub const KEY_DOWN: char = 'j';
pub const KEY_UP: char = 'k';
pub const KEY_RIGHT: char = 'l';
pub const KEY_EXECUTE: char = 'L';
pub const KEY_CMDMODE: char = ':';
pub const KEY_CMDENTER: char = '\r';
