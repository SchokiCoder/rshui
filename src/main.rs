/* Copyright (C) 2023 Andy Frank Schoknecht
 * Use of this source code is governed by the BSD-3-Clause
 * license, that can be found in the LICENSE file.
 */

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
const SIGTSTP: char = '\x32';

fn draw_menu(menu: &Menu, cursor: usize)
{
	for i in 0..menu.entries.len() {
		if i == cursor {
			print!("{}{}",
			       color::Fg(color::Black),
			       color::Bg(color::White));
			print!("{}{}\n", ENTRY_PREPEND, menu.entries[i].caption);
			print!("{}{}",
			       color::Fg(color::Reset),
			       color::Bg(color::Reset));
		} else {
			print!("{}{}\n", ENTRY_PREPEND, menu.entries[i].caption);
		}
	}
}

fn draw_lower(stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
              term_w: u16,
              term_h: u16,
              feedback: &Option<String>)
{
	let y: u16;

	stdout.activate_raw_mode().unwrap();
	(_, y) = stdout.cursor_pos().unwrap();
	stdout.suspend_raw_mode().unwrap();

	for _ in y..term_h {
		print!("\n");
	}
	
	print!(":");

	let fb_str = match feedback {
		Some(x) => {
			x
		}
		None => {
			return;
		}
	};

	if fb_str.len() > term_w as usize {
		return;
	}
	
	print!("{}", fb_str);
}

fn main()
{
	let mut cursor: usize = 0;
	let mut cmdoutput: Option<std::process::Output> = None;
	let mut feedback: Option<String> = None;
	let mut input: [u8; 1] = [0];
	let mut stdin: std::io::Stdin;
	let mut stdout: termion::cursor::HideCursor<
		termion::raw::RawTerminal<std::io::Stdout>>;
	let mut term_w: u16;
	let mut term_h: u16;

	stdout = HideCursor::from(std::io::stdout().into_raw_mode().unwrap());
	stdin = std::io::stdin();
	
	'mainloop: loop {		
		(term_w, term_h) = termion::terminal_size().unwrap();

		print!("{}", clear::All);
		print!("{}", cursor::Goto(1, 1));
		stdout.suspend_raw_mode().unwrap();
		
		print!("{}\n", HEADER);
		print!("{}\n", MENU_MAIN.title);
		draw_menu(&MENU_MAIN, cursor);

		match cmdoutput {
			Some(value) => {
			if value.stderr.len() > 0 {
				feedback = Some(String::from_utf8(value.stderr).unwrap());
			} else {
				feedback = Some(String::from_utf8(value.stdout).unwrap());
			}
			}
			
			None => {}
		}
		cmdoutput = None;
		draw_lower(&mut stdout, term_w, term_h, &feedback);
		
		stdout.flush().unwrap();
		stdout.activate_raw_mode().unwrap();

		stdin.read_exact(&mut input).unwrap();

		match input[0] as char {
			SIGINT | SIGTSTP | 'q' => {
			break 'mainloop;
			}
		
			'j' => {
			if cursor < (MENU_MAIN.entries.len() - 1) {
				cursor += 1;
			}
			}
		
			'k' => {
			if cursor > 0 {
				cursor -= 1;
			}
			}

			'\r' => {
			match MENU_MAIN.entries[cursor].content {
				EntryContent::Shell(cmdstr) => {
				cmdoutput = Some(Command::new("sh")
					.arg("-c")
					.arg(cmdstr)
					.output()
					.unwrap());
				}

				_ => {}
			}
			}

			_ => {}
		}
	}
}
