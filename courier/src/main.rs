// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023 - 2024  Andy Frank Schoknecht

mod config;

use crate::config::CouCfg;

use common::*;
use common::config::ComCfg;
use std::io::{Read, Write};
use termion::{clear, cursor};
use termion::cursor::{HideCursor};
use termion::raw::{IntoRawMode, RawTerminal};

fn draw_content(content: &str)
{
	print!("{}", content)
}

fn handle_key(key:       char,
              active:    &mut bool,
              comcfg:    &ComCfg,
//              cmdline:   &mut String,
//              cmdmode:   &mut bool,
//              feedback:  &mut Option<String>
		)
{
	if key == common::SIGINT ||
	   key == common::SIGTSTP ||
	   key == comcfg.keys.quit {
		*active = false;
	}
}

fn main()
{
	let comcfg = ComCfg::from_file();
	let coucfg = CouCfg::from_file();
	let title: String;

	let mut active = true;
	let cmdline: String = String::new();
	let cmdmode: bool = false;
	let content: String;
	let feedback: Option<String> = None;
	let mut input: [u8; 1] = [0];
	let mut stdin: std::io::Stdin;
	let mut stdout: HideCursor<RawTerminal<std::io::Stdout>>;
	let mut term_w: u16;
	let mut term_h: u16;
	
	content = "test1\ntest2\ntest3\n".to_string();
	stdin = std::io::stdin();
	stdout = HideCursor::from(std::io::stdout().into_raw_mode().unwrap());
	title = "Your ad could be here, for now!".to_string();

	while active {
		(term_w, term_h) = termion::terminal_size().unwrap();

		print!("{}", clear::All);
		print!("{}", cursor::Goto(1, 1));
		stdout.suspend_raw_mode().unwrap();

		draw_upper(&comcfg, &coucfg.header, &title);
		draw_content(&content);
		draw_lower(&comcfg,
			   &cmdline,
			   &cmdmode,
			   &feedback,
			   &mut stdout,
			   term_w,
			   term_h);
		
		stdout.flush().expect("stdout flush failed");
		stdout.activate_raw_mode().unwrap();
		
		stdin.read_exact(&mut input).expect("keyboard read failed");

		handle_key(input[0] as char, &mut active, &comcfg);
	}
}
