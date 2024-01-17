// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023 - 2024  Andy Frank Schoknecht

use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub struct FgColor {
	pub active: bool,
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

#[derive(Deserialize)]
pub struct BgColor {
	pub active: bool,
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

impl std::fmt::Display for FgColor {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		if self.active {
			write!(f, "\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
		} else {
			write!(f, "\x1b[39m")
		}
	}
}

impl std::fmt::Display for BgColor {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		if self.active {
			write!(f, "\x1b[48;2;{};{};{}m", self.r, self.g, self.b)
		} else {
			write!(f, "\x1b[49m")
		}
	}
}

#[derive(Deserialize)]
pub struct TextColor {
	pub fg: FgColor,
	pub bg: BgColor,
}
