// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023  Andy Frank Schoknecht

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
