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
	if get_needed_lines(fb_str, term_w as usize) != 1 {
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

pub fn draw_upper(comcfg: &ComCfg, header: &str, title: &str)
{
	print!("{}{}{}{}{}\n",
	       comcfg.colors.header.fg,
	       comcfg.colors.header.bg,
	       header,
	       comcfg.colors.std.fg,
	       comcfg.colors.std.bg);

	print!("{}{}{}{}{}\n",
	       comcfg.colors.title.fg,
	       comcfg.colors.title.bg,
	       title,
	       comcfg.colors.std.fg,
	       comcfg.colors.std.bg);
}

pub fn get_needed_lines(s: &str, line_width: usize) -> usize
{
	let mut ret: usize = 1;
	let mut x: usize = 0;

	if s.len() == 0 {
		return 0;
	}

	for c in s.bytes() {
		match c as char {
		'\r' | '\n' => {
			ret += 1;
			x = 0;
			}

		_ => { x += 1; }
		}

		if x > line_width {
			x = 0;
			ret += 1;
		}
	}

	ret
}
