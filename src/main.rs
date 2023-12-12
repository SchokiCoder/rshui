/* Copyright (C) 2023 Andy Frank Schoknecht
 * Use of this source code is governed by the BSD-3-Clause
 * license, that can be found in the LICENSE file.
 */

mod menu;
mod config;

use crate::menu::*;
use crate::config::*;

use std::io::{Read, Write};
use termion::raw::IntoRawMode;
use termion::clear;
use termion::cursor::HideCursor;

const SIGINT:  char = '\x03';
const SIGTSTP: char = '\x32';

fn draw_menu(menu: &Menu)
{
	for entry in menu.entries {
		print!("{}{}", ENTRY_PREPEND, entry.caption);
	}
}

fn main()
{
	let stdout = std::io::stdout().into_raw_mode().unwrap();
	let mut stdin = std::io::stdin();
	let mut input = Vec::<u8>::new();
	let read_bytes: usize;

	let mut stdout = HideCursor::from(stdout);
	
	read_bytes = stdin.read_to_end(&mut input).unwrap();
	'mainloop: for i in 0..read_bytes {
		match input[i] as char {
		SIGINT | SIGTSTP | 'q' => {
			break 'mainloop;
		}
		
		_ => {}
		}
		
		print!("{}", clear::All);
		print!("{}", HEADER);
		print!("{}", MENU_MAIN.title);
		draw_menu(&MENU_MAIN);
		
		stdout.flush().unwrap();
	}
}
