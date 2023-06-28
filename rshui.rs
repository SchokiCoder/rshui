/* Copyright (C) 2023 Andy Frank Schoknecht
 * Use of this source code is governed by the BSD-3-Clause
 * license, that can be found in the LICENSE file.
 */

mod menu;
mod config;

use std::os::unix::io::FromRawFd;

use crate::menu::*;
use crate::config::*;

const SIGINT:  char = '\x03';
const SIGTSTP: char = '\x32';

const SEQ_CLEAR:     &str = "\x1b[2J";
const SEQ_CRSR_HIDE: &str = "\033[?25l";
const SEQ_CRSR_SHOW: &str = "\033[?25h";

extern "C"
{
	// init
	setbuf(stdout, NULL);				// probably needs no ffi
	tcgetattr(STDIN_FILENO, &orig);
	raw = orig;
	raw.c_lflag &= ~(ECHO | ICANON | ISIG);
	tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw);

	// quit
	tcsetattr(STDIN_FILENO, TCSAFLUSH, &orig);
}

fn draw_menu(menu: &Menu)
{
	for entry in menu.entries {
		println!("{}{}", ENTRY_PREPEND, entry.caption);
	}
}

fn main()
{
	let stdin = std::io::stdin();
	
	// get term info and set raw mode
	// TODO wip (see extern at top for what is needed)
	let stdout = std::fs::File::from_raw_fd(1);
	print!("{}", SEQ_CRSR_HIDE);
	
	'mainloop: for key in stdin.keys() {
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
