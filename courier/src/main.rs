// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023 - 2024  Andy Frank Schoknecht

mod config;

use crate::config::CouCfg;

use common::*;
use common::config::ComCfg;
use std::io::{Read};
use termion::{clear, cursor};

fn draw_content(content: &str)
{
	print!("{}", content)
}

fn main()
{
	let comcfg = ComCfg::from_file();
	let coucfg = CouCfg::from_file();
	
	let mut active = true;
	let content: String;
	let mut input: [u8; 1] = [0];
	let mut stdin: std::io::Stdin;
	let title: String;
	
	content = "test1\ntest2\ntest3\n".to_string();
	stdin = std::io::stdin();
	title = "Your ad could be here, for now!".to_string();

	while active {
		print!("{}", clear::All);
		print!("{}", cursor::Goto(1, 1));

		draw_upper(&comcfg, &coucfg.header, &title);
		draw_content(&content);

		stdin.read_exact(&mut input).expect("keyboard read failed");

		if input[0] as char == 'q' {
			active = false;
		}
	}
}
