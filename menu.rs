/* Copyright (C) 2023 Andy Frank Schoknecht
 * Use of this source code is governed by the BSD-3-Clause
 * license, that can be found in the LICENSE file.
 */

#[allow(dead_code)]

pub enum EntryContent<'a> {
	Shell(&'a str),
	Rust,
	Menu(&'a Menu<'a>),
}

pub struct Entry<'a> {
	pub caption: &'a str,
	pub content: EntryContent<'a>,
}

pub struct Menu<'a> {
	pub title:   &'a str,
	pub entries:	&'a [Entry<'a>],
}
