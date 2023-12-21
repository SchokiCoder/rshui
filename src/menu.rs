// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023  Andy Frank Schoknecht

#[allow(dead_code)]

pub enum EntryContent<'a> {
	Menu(&'a Menu<'a>),
	Rust,
	Shell(&'a str),
}

pub struct Entry<'a> {
	pub caption: &'a str,
	pub content: EntryContent<'a>,
}

pub struct Menu<'a> {
	pub title:   &'a str,
	pub entries: &'a [Entry<'a>],
}
