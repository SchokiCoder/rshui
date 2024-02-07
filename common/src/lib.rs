// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023 - 2024  Andy Frank Schoknecht

pub mod color;
pub mod config;

use crate::config::ComCfg;
use termion::cursor::DetectCursorPos;

pub const SIGINT:  char = '\x03';
pub const SIGTSTP: char = '\x04';

pub fn draw_feedback(feedback: &Option<String>, comcfg: &ComCfg, term_w: u16)
{
	let fb_str = match feedback {
	Some(x) => {
		x
		}
	
	None => {
		return;
		}
	};

	let fb_str = fb_str.trim_end();
	if split_by_lines(fb_str, term_w).len() != 1 {
		return;
	}

	print!("{}{}{}{}{}",
	       comcfg.colors.feedback.fg,
	       comcfg.colors.feedback.bg,
	       fb_str,
	       comcfg.colors.std.fg,
	       comcfg.colors.std.bg);
}

pub fn draw_lower(comcfg: &ComCfg,
	      cmdline: &String,
              cmdmode: &bool,
              feedback: &Option<String>,
              stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
              term_w: u16,
              term_h: u16)
{
	let y: u16;

	stdout.activate_raw_mode().unwrap();
	(_, y) = stdout.cursor_pos().unwrap();
	stdout.suspend_raw_mode().unwrap();

	for _ in y..term_h {
		print!("\n");
	}
	
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
