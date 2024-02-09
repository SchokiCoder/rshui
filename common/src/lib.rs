// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023 - 2024  Andy Frank Schoknecht

pub mod color;
pub mod config;

use crate::config::ComCfg;
use termion::cursor;

pub const SIGINT:  char = '\x03';
pub const SIGTSTP: char = '\x04';

pub fn draw_feedback(feedback: &mut String, comcfg: &ComCfg, term_w: u16)
{
	*feedback = feedback.trim_end().to_string();
	if split_by_lines(feedback, term_w).len() != 1 {
		return;
	}

	print!("{}{}{}{}{}",
	       comcfg.colors.feedback.fg,
	       comcfg.colors.feedback.bg,
	       feedback,
	       comcfg.colors.std.fg,
	       comcfg.colors.std.bg);
}

pub fn draw_lower(comcfg: &ComCfg,
	      cmdline: &String,
              cmdmode: &bool,
              feedback: &mut String,
              term_w: u16,
              term_h: u16)
{
	print!("{}", cursor::Goto(1, term_h));
	
	print!("{}{}:{}{}",
	       comcfg.colors.feedback.fg,
	       comcfg.colors.feedback.bg,
	       comcfg.colors.std.fg,
	       comcfg.colors.std.bg);

	if *cmdmode {
		print!("{}{}{}{}{}",
		       comcfg.colors.cmdline.fg,
		       comcfg.colors.cmdline.bg,
		       cmdline,
		       comcfg.colors.std.fg,
		       comcfg.colors.std.bg);
	} else {
		draw_feedback(feedback, comcfg, term_w);
	}
}

pub fn draw_upper(comcfg: &ComCfg,
                  header_lines: &Vec<String>,
                  title_lines: &Vec<String>)
{
	print!("{}", cursor::Goto(1, 1));

	for line in header_lines {
		print!("{}{}{}{}{}\n",
		       comcfg.colors.header.fg,
		       comcfg.colors.header.bg,
		       line,
		       comcfg.colors.std.fg,
		       comcfg.colors.std.bg);
	}
	
	for line in title_lines {
		print!("{}{}{}{}{}\n",
		       comcfg.colors.title.fg,
		       comcfg.colors.title.bg,
		       line,
		       comcfg.colors.std.fg,
		       comcfg.colors.std.bg);
	}
}

pub fn split_by_lines(s: &str, line_width: u16) -> Vec<String>
{
	let mut cut: usize = 0;
	let mut ret = Vec::<String>::new();
	let mut x: usize = 0;

	if s.len() == 0 {
		return ret;
	}

	for c in s.bytes() {
		x += 1;
		
		match c as char {
		'\r' | '\n' => {
			ret.push(s[cut..(cut + x - 1)].to_string());
			cut += x;
			x = 0;
		}

		_ => {}
		}

		if x > line_width as usize {
			ret.push(s[cut..(cut + x - 1)].to_string());
			cut += x;
			x = 0;
		}
	}
	
	ret.push(s[cut..(cut + x)].to_string());
	return ret;
}
