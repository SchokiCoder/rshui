// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023  Andy Frank Schoknecht

mod menu;
mod color;
mod config;

use crate::config::Config;
use crate::menu::*;

use std::io::{Read, Write};
use std::process::Command;
use termion::{clear, cursor};
use termion::cursor::{DetectCursorPos, HideCursor};
use termion::raw::IntoRawMode;

const SIGINT:  char = '\x03';
const SIGTSTP: char = '\x04';

fn cmdoutput_to_feedback(cmdoutput: Option<std::process::Output>,
                         feedback:  &mut Option<String>)
{
	match cmdoutput {
	Some(value) => {
		if value.stderr.len() > 0 {
			*feedback = Some(String::from_utf8(value.stderr).unwrap());
		} else {
			*feedback = Some(String::from_utf8(value.stdout).unwrap());
		}
	}

	None => {}
	};
}

fn draw_feedback(feedback: &Option<String>, cfg: &Config, term_w: u16)
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
	       cfg.colors.feedback.fg,
	       cfg.colors.feedback.bg,
	       fb_str,
	       cfg.colors.std.fg,
	       cfg.colors.std.bg);
}

fn draw_lower(cfg: &Config,
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
	       cfg.colors.feedback.fg,
	       cfg.colors.feedback.bg,
	       cfg.colors.std.fg,
	       cfg.colors.std.bg);

	if *cmdmode {
		print!("{}{}{}{}{}",
		       cfg.colors.cmdline.fg,
		       cfg.colors.cmdline.bg,
		       cmdline,
		       cfg.colors.std.fg,
		       cfg.colors.std.bg);
	} else {
		draw_feedback(feedback, cfg, term_w);
	}
}

fn draw_menu(menu: &Menu, cfg: &Config, cursor: usize)
{
	let mut prefix:  &str;
	let mut caption: &str;
	let mut postfix: &str;

	for i in 0..menu.entries.len() {
		caption = menu.entries[i].caption;
		
		match menu.entries[i].content {
		EntryContent::Menu(_) => {
			prefix  = cfg.entry_menu_prefix;
			postfix = cfg.entry_menu_postfix;
		}

		EntryContent::Shell(_) => {
			prefix  = cfg.entry_shell_prefix;
			postfix = cfg.entry_shell_postfix;
		}
		}
		
		if i == cursor {
			print!("{}{}{}{}{}{}{}\n",
			       cfg.colors.entry_hover.fg,
			       cfg.colors.entry_hover.bg,
			       prefix,
			       caption,
			       postfix,
			       cfg.colors.std.fg,
			       cfg.colors.std.bg);
		} else {
			print!("{}{}{}{}{}{}{}\n",
			       cfg.colors.entry.fg,
			       cfg.colors.entry.bg,
			       prefix,
			       caption,
			       postfix,
			       cfg.colors.std.fg,
			       cfg.colors.std.bg);
		}
	}
}

fn draw_upper(cfg: &Config, title: &str)
{
	print!("{}{}{}{}{}\n",
	       cfg.colors.header.fg,
	       cfg.colors.header.bg,
	       cfg.header,
	       cfg.colors.std.fg,
	       cfg.colors.std.bg);

	print!("{}{}{}{}{}\n",
	       cfg.colors.title.fg,
	       cfg.colors.title.bg,
	       title,
	       cfg.colors.std.fg,
	       cfg.colors.std.bg);
}

fn get_needed_lines(s: &str, line_width: usize) -> usize
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

fn handle_cmd(cmdline: &mut String,
              active: &mut bool,
              cursor: &mut usize,
              feedback: &mut Option<String>,
              menu_path: &mut Vec<&Menu>)
{
	let cur_menu: &Menu = menu_path[menu_path.len() - 1];
	let res_num: Result<usize, std::num::ParseIntError>;
	let num: usize;

	match cmdline as &str {
	"q" | "quit" | "exit" => {
		*active = false;
	}

	_ => {
		res_num = usize::from_str_radix(cmdline.as_ref(), 10);
		
		if res_num.is_ok() {
			num = res_num.unwrap();
			
			if num > 0 {
				if num > cur_menu.entries.len() {
					*cursor = cur_menu.entries.len() - 1;
				} else {
					*cursor = num - 1;
				}
			}
		} else {
			*feedback = Some(format!("Command \"{}\" not recognised",
			                         cmdline));
		}
	}
	}
	
	cmdline.clear();
}

fn handle_key(key:       char,
              active:    &mut bool,
              cfg:       &Config,
              cmdline:   &mut String,
              cmdmode:   &mut bool,
              cmdoutput: &mut Option<std::process::Output>,
              cursor:    &mut usize,
              feedback:  &mut Option<String>,
              menu_path: &mut Vec<&Menu>)
{
	let cur_menu: &Menu = menu_path[menu_path.len() - 1];

	if *cmdmode {
		match key {
		SIGINT | SIGTSTP => {
			*cmdmode = false;
		}

		cfg.keys.cmdenter => {
			handle_cmd(cmdline, active, cursor, feedback, menu_path);
			*cmdmode = false;
		}

		_ => {
			cmdline.push(key);
		}
		}
		
		return;
	}

	match key {
	SIGINT | SIGTSTP | 'q' => {
		*active = false;
	}

	cfg.keys.down => {
		if *cursor < (cur_menu.entries.len() - 1) {
			*cursor += 1;
		}
	}

	cfg.keys.up => {
		if *cursor > 0 {
			*cursor -= 1;
		}
	}
	
	cfg.keys.right => {
		match cur_menu.entries[*cursor].content {
		EntryContent::Menu(m) => {
			menu_path.push(&m);
		}
		
		_ => {}
		}
	}
	
	cfg.keys.left => {
		if menu_path.len() > 1 {
			menu_path.pop();
		}
	}

	cfg.keys.execute => {
		match cur_menu.entries[*cursor].content {
		EntryContent::Shell(cmdstr) => {
			*cmdoutput = Some(Command::new("sh")
				.arg("-c")
				.arg(cmdstr)
				.output()
				.unwrap());
		}

		_ => {}
		}
	}
	
	cfg.keys.cmdmode => {
		if *cmdmode == false {
			*cmdmode = true;
		}
	}

	_ => {}
	}
}

fn main()
{
	let cfg = config::Config::from_file();
	
	let mut active: bool = true;
	let mut cursor: usize = 0;
	let mut cmdline: String = String::new();
	let mut cmdmode: bool = false;
	let mut cmdoutput: Option<std::process::Output> = None;
	let mut feedback: Option<String> = None;
	let mut input: [u8; 1] = [0];
	let mut stdin: std::io::Stdin;
	let mut stdout: termion::cursor::HideCursor<
		termion::raw::RawTerminal<std::io::Stdout>>;
	let mut term_w: u16;
	let mut term_h: u16;
	let mut menu_path: Vec<&Menu> = vec![&cfg.menus[0]];

	stdout = HideCursor::from(std::io::stdout().into_raw_mode().unwrap());
	stdin = std::io::stdin();
	
	while active {
		let cur_menu: &Menu = menu_path[menu_path.len() - 1];

		(term_w, term_h) = termion::terminal_size().unwrap();

		print!("{}", clear::All);
		print!("{}", cursor::Goto(1, 1));
		stdout.suspend_raw_mode().unwrap();

		draw_upper(&cfg, cur_menu.title);
		draw_menu(&cur_menu, &cfg, cursor);

		cmdoutput_to_feedback(cmdoutput, &mut feedback);
		cmdoutput = None;
		draw_lower(&cfg,
			   &cmdline,
			   &cmdmode,
			   &feedback,
			   &mut stdout,
			   term_w,
			   term_h);
		
		stdout.flush().unwrap();
		stdout.activate_raw_mode().unwrap();

		stdin.read_exact(&mut input).unwrap();
		handle_key(input[0] as char,
		           &mut active,
		           &cfg,
		           &mut cmdline,
		           &mut cmdmode,
		           &mut cmdoutput,
		           &mut cursor,
		           &mut feedback,
		           &mut menu_path);
	}
}
