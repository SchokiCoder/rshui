// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023  Andy Frank Schoknecht

mod menu;
mod config;

use crate::menu::*;
use crate::config::*;

use std::io::{Read, Write};
use std::process::Command;
use termion::{clear, color, cursor};
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

fn draw_feedback(feedback: &Option<String>, term_w: u16)
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
	
	print!("{}{}{}",
	       color::Fg(color::LightBlack),
	       fb_str,
	       color::Fg(color::Reset));
}

fn draw_lower(cmdline: &String,
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
	
	print!("{}:{}", color::Fg(color::LightBlack), color::Fg(color::Reset));

	if *cmdmode {
		print!("{}{}{}",
		       color::Fg(color::LightBlack),
		       cmdline,
		       color::Fg(color::Reset));
	} else {
		draw_feedback(feedback, term_w);
	}
}

fn draw_menu(menu: &Menu, cursor: usize)
{
	let mut prefix:  &str;
	let mut caption: &str;
	let mut postfix: &str;

	for i in 0..menu.entries.len() {
		caption = menu.entries[i].caption;
		
		match menu.entries[i].content {
		EntryContent::Menu(_) => {
			prefix  = ET_MENU_PREFIX;
			postfix = ET_MENU_POSTFIX;
		}

		EntryContent::Rust => {
			prefix  = ET_RS_PREFIX;
			postfix = ET_RS_POSTFIX;
		}

		EntryContent::Shell(_) => {
			prefix  = ET_SH_PREFIX;
			postfix = ET_SH_POSTFIX;
		}
		}
		
		if i == cursor {
			print!("{}{}",
			       color::Fg(color::Black),
			       color::Bg(color::White));
			print!("{}{}{}\n", prefix, caption, postfix);
			print!("{}{}",
			       color::Fg(color::Reset),
			       color::Bg(color::Reset));
		} else {
			print!("{}{}{}\n", prefix, caption, postfix);
		}
	}
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
              cmdline:   &mut String,
              cmdmode:   &mut bool,
              cmdoutput: &mut Option<std::process::Output>,
              cursor:    &mut usize,
              feedback: &mut Option<String>,
              menu_path: &mut Vec<&Menu>)
{
	let cur_menu: &Menu = menu_path[menu_path.len() - 1];

	if *cmdmode {
		match key {
		SIGINT | SIGTSTP => {
			*cmdmode = false;
		}

		'\r' => {
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

	'j' => {
		if *cursor < (cur_menu.entries.len() - 1) {
			*cursor += 1;
		}
	}

	'k' => {
		if *cursor > 0 {
			*cursor -= 1;
		}
	}
	
	'l' => {
		match cur_menu.entries[*cursor].content {
		EntryContent::Menu(m) => {
			menu_path.push(&m);
		}
		
		_ => {}
		}
	}
	
	'h' => {
		if menu_path.len() > 1 {
			menu_path.pop();
		}
	}

	'L' => {
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
	
	':' => {
		if *cmdmode == false {
			*cmdmode = true;
		}
	}

	_ => {}
	}
}

fn main()
{
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
	let mut menu_path: Vec<&Menu> = vec![&MENU_MAIN];

	stdout = HideCursor::from(std::io::stdout().into_raw_mode().unwrap());
	stdin = std::io::stdin();
	
	while active {
		let cur_menu: &Menu = menu_path[menu_path.len() - 1];

		(term_w, term_h) = termion::terminal_size().unwrap();

		print!("{}", clear::All);
		print!("{}", cursor::Goto(1, 1));
		stdout.suspend_raw_mode().unwrap();
		
		print!("{}\n", HEADER);
		print!("{}\n", cur_menu.title);
		draw_menu(&cur_menu, cursor);

		cmdoutput_to_feedback(cmdoutput, &mut feedback);
		cmdoutput = None;
		draw_lower(&cmdline,
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
		           &mut cmdline,
		           &mut cmdmode,
		           &mut cmdoutput,
		           &mut cursor,
		           &mut feedback,
		           &mut menu_path);
	}
}
