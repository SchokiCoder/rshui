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

#[must_use]
fn handle_cmd(cmdline: &mut String,
              active: &mut bool)
              -> Option<String> // feedback is returned
{
	let mut ret: Option<String> = None;
	
	match cmdline as &str {
	"q" | "quit" | "exit" => {
		*active = false;
		}

	_ => {
/*	TODO change for scroll later
	
		match usize::from_str_radix(cmdline.as_ref(), 10) {
		Ok(num) => {
			if num > 0 {
				if num > cur_menu.entries.len() {
					*cursor = cur_menu.entries.len() - 1;
				} else {
					*cursor = num - 1;
				}
			}
			},

		Err(_) => {
			return Some(format!("Command \"{}\" not recognised",
			                    cmdline));
			},
		}*/
		ret = Some(format!("Command \"{}\" not recognised", cmdline));
		}
	}
	
	cmdline.clear();
	return ret;
}

fn handle_key(key:       char,
              active:    &mut bool,
              comcfg:    &ComCfg,
              cmdline:   &mut String,
              cmdmode:   &mut bool,
              feedback:  &mut Option<String>)
{
	if *cmdmode {
		if key == common::SIGINT || key == common::SIGTSTP {
			*cmdmode = false;
		} else if key == comcfg.keys.cmdenter {
			*feedback = handle_cmd(cmdline, active);
			*cmdmode = false;
		} else {
			cmdline.push(key);
		}
		
		return;
	}
	
	if key == common::SIGINT ||
	   key == common::SIGTSTP ||
	   key == comcfg.keys.quit {
		*active = false;
	} else if key == comcfg.keys.cmdmode {
		*cmdmode = true;
	}
}

fn main()
{
	let comcfg = ComCfg::from_file();
	let coucfg = CouCfg::from_file();
	let title: String;

	let mut active = true;
	let mut cmdline: String = String::new();
	let mut cmdmode: bool = false;
	let content: String;
	let mut feedback: Option<String> = None;
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

		handle_key(input[0] as char,
		           &mut active,
		           &comcfg,
		           &mut cmdline,
		           &mut cmdmode,
		           &mut feedback);
	}
}
