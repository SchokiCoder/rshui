/* Copyright (C) 2023 Andy Frank Schoknecht
 * Use of this source code is governed by the BSD-3-Clause
 * license, that can be found in the LICENSE file.
 */

mod menu;
mod config;

use crate::menu::*;
use crate::config::*;

use std::io::{Read, Write};
use termion::clear;
use termion::cursor;
use termion::cursor::{HideCursor};
use termion::raw::IntoRawMode;

const SIGINT:  char = '\x03';
const SIGTSTP: char = '\x32';

fn draw_menu(menu: &Menu)
{
	for entry in menu.entries {
		print!("{}{}\n", ENTRY_PREPEND, entry.caption);
	}
}

fn main()
{
	let mut stdout: termion::cursor::HideCursor<
		termion::raw::RawTerminal<std::io::Stdout>>;
	let mut stdin: std::io::Stdin;
	let mut input: [u8; 1] = [0];

	stdout = HideCursor::from(std::io::stdout().into_raw_mode().unwrap());
	stdin = std::io::stdin();
	
	'mainloop: loop {		
		print!("{}", clear::All);
		print!("{}", cursor::Goto(1, 1));
		stdout.suspend_raw_mode().unwrap();
		
		print!("{}\n", HEADER);
		print!("{}\n", MENU_MAIN.title);
		draw_menu(&MENU_MAIN);
		
		stdout.flush().unwrap();
		stdout.activate_raw_mode().unwrap();

		stdin.read_exact(&mut input).unwrap();

		match input[0] as char {
		SIGINT | SIGTSTP | 'q' => {
			break 'mainloop;
		}
		
		_ => {}
		}
	}
}
