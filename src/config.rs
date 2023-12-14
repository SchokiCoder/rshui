/* Copyright (C) 2023 Andy Frank Schoknecht
 * Use of this source code is governed by the BSD-3-Clause
 * license, that can be found in the LICENSE file.
 */

use crate::menu::*;

pub const HEADER: &str = "Example config\n";

pub const MENU_MAIN: Menu = Menu {
	title: "Main Menu\n\
		---------",
	entries: &[
		Entry {
			caption: "Show current user",
			content: EntryContent::Shell("echo \"$USER\""),
		},

		Entry {
			caption: "Do a funny",
			content: EntryContent::Shell("echo \"amogus\""),
		},

		Entry {
			caption: "Clear ~/temp",
			content: EntryContent::Shell("echo \"\" > ~/temp"),
		},
	],
};

pub const ENTRY_PREPEND: &str = "> ";
