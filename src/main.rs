/* Copyright (C) 2023 Andy Frank Schoknecht
 * Use of this source code is governed by the BSD-3-Clause
 * license, that can be found in the LICENSE file.
 */

mod menu;
mod config;

use crate::menu::*;
use crate::config::*;

use termion::raw::IntoRawMode;
use termion::cursor::HideCursor;

const SIGINT:  char = '\x03';
const SIGTSTP: char = '\x32';

const SEQ_CLEAR:     &str = "\x1b[2J";
const SEQ_CRSR_HIDE: &str = "\033[?25l";
const SEQ_CRSR_SHOW: &str = "\033[?25h";

fn draw_menu(menu: &Menu)
{
	for entry in menu.entries {
		println!("{}{}", ENTRY_PREPEND, entry.caption);
	}
}

fn main()
{
	let stdout = std::io::stdout().into_raw_mode().unwrap();
	let stdin = std::io::stdin();
	let cursor_hider = HideCursor::from(stdout);
	let mut input = Vec::<u8>::new();
	
	input = stdin.read_to_end(&mut input);
	'mainloop: for key in input {
		if !key.is_ok() {
			continue;
		}
		
		match key.unwrap() {
		(SIGINT, SIGTSTP, 'q') => {
			break 'mainloop;
		}
		}
		
		print!("{}", SEQ_CLEAR);
		
		println!("{}", HEADER);
		println!("{}", MENU_MAIN.title);
		
		draw_menu(&MENU_MAIN);
		
		break 'mainloop;
	}
	
	// restore original terminal settings
	print!("{}", SEQ_CRSR_SHOW);
}
