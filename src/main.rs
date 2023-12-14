/* Copyright (C) 2023 Andy Frank Schoknecht
 * Use of this source code is governed by the BSD-3-Clause
 * license, that can be found in the LICENSE file.
 */

mod menu;
mod config;

use crate::menu::*;
use crate::config::*;

use std::io::{Read, Write};
use termion::{clear, color, cursor};
use termion::cursor::{HideCursor};
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

fn main()
{
	let mut stdout: termion::cursor::HideCursor<
		termion::raw::RawTerminal<std::io::Stdout>>;
	let mut stdin: std::io::Stdin;
	let mut input: [u8; 1] = [0];
	let mut cursor: usize = 0;

	stdout = HideCursor::from(std::io::stdout().into_raw_mode().unwrap());
	stdin = std::io::stdin();
	
	'mainloop: loop {		
		print!("{}", clear::All);
		print!("{}", cursor::Goto(1, 1));
		stdout.suspend_raw_mode().unwrap();
		
		print!("{}\n", HEADER);
		print!("{}\n", MENU_MAIN.title);
		draw_menu(&MENU_MAIN, cursor);
		
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
		_ => {}
		}
	}
}
