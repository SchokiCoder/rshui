// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023  Andy Frank Schoknecht

use serde::Deserialize;

#[derive(Deserialize)]
pub enum EntryContent<'a> {
	Menu(Menu<'a>),
	Shell(&'a str),
}

#[derive(Deserialize)]
pub struct Entry<'a> {
	pub caption: &'a str,
	pub content: EntryContent<'a>,
}

#[derive(Deserialize)]
pub struct Menu<'a> {
	pub name:    &'a str,
	pub title:   &'a str,
	pub entries: Vec<Entry<'a>>,
}
