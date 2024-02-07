// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023 - 2024  Andy Frank Schoknecht

mod config;

use crate::config::CouCfg;

use common::*;
use common::config::ComCfg;
use std::io::{Read, Write};
use termion::{clear, cursor};
use termion::raw::{IntoRawMode};

fn draw_content(content_height: usize,
                comcfg:         &ComCfg,
                coucfg:         &CouCfg,
                content:        &Vec<String>,
                scroll:         usize)
{	
	let mut line_index_begin: usize;
	let mut line_index_end: usize;

	if content_height < content.len() {
		line_index_begin = scroll;
		line_index_end = scroll + content_height;
		
		if line_index_end >= content.len() {
			line_index_begin -= line_index_end - content.len();
			line_index_end -= line_index_end - content.len();
		}
	} else {
		line_index_begin = 0;
		line_index_end = content.len();
	} 
	
	for i in line_index_begin..line_index_end {
		print!("{}{}{}{}{}\n",
		       coucfg.colors.content.fg,
		       coucfg.colors.content.bg,
		       content[i],
		       comcfg.colors.std.fg,
		       comcfg.colors.std.bg);
	}
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
	}}
	
	cmdline.clear();
	return ret;
}

fn handle_key(key:          char,
              active:       &mut bool,
              comcfg:       &ComCfg,
              cmdline:      &mut String,
              cmdmode:      &mut bool,
              feedback:     &mut Option<String>,
              scroll_limit: usize,
              scroll:       &mut usize)
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
	} else if key == comcfg.keys.up {
		if *scroll > 0 {
			*scroll -= 1;
		}
	} else if key == comcfg.keys.down {
		if *scroll < scroll_limit {
			*scroll += 1;
		}
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
	let mut content = Vec::<String>::new();
	let mut content_height: usize;
	let mut feedback: Option<String> = None;
	let mut header_lines = Vec::<String>::new();
	let mut input: [u8; 1] = [0];
	let mut scroll: usize = 0;
	let mut stdin: std::io::Stdin;
	let mut term_w: u16 = 0;
	let mut term_w_old: u16;
	let mut term_h: u16;
	let mut title_lines = Vec::<String>::new();

	stdin = std::io::stdin();
	let mut stdout = std::io::stdout().into_raw_mode().unwrap();
	// TODO temp val
	title = "Your ad could be here, for now!".to_string();

	while active {
		term_w_old = term_w;
		(term_w, term_h) = termion::terminal_size().unwrap();
		if term_w_old != term_w {
			// TODO temp val
			content = split_by_lines("\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test1\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n\
test\n", term_w);
			header_lines = split_by_lines(&coucfg.header, term_w);
			title_lines = split_by_lines(&title, term_w);
		}
		content_height = term_h as usize -
		                 header_lines.len() -
		                 title_lines.len() -
		                 1 - 1;

		print!("{}", clear::All);
		print!("{}", cursor::Goto(1, 1));
		stdout.suspend_raw_mode().unwrap();

		draw_upper(&comcfg, &header_lines, &title_lines);

		draw_content(content_height,
		             &comcfg,
		             &coucfg,
		             &content,
		             scroll);

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
		           &mut feedback,
		           content.len() - content_height,
		           &mut scroll);
	}
}
