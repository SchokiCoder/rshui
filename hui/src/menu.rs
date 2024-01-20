// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023 - 2024  Andy Frank Schoknecht

use serde::Deserialize;

#[derive(Deserialize)]
pub enum EntryContent {
	Menu(String),
	Shell(String),
	ShellSession(String),
}

#[derive(Deserialize)]
pub struct Entry {
	pub caption: String,
	pub content: EntryContent,
}

#[derive(Deserialize)]
pub struct Menu {
	pub title:   String,
	pub entries: Vec<Entry>,
}
