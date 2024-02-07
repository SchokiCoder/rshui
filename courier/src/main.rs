// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023 - 2024  Andy Frank Schoknecht

mod config;

use crate::config::CouCfg;

use common::*;
use common::config::ComCfg;
use std::env;
use std::fs;
use std::io::{Read, Write};
use termion::{clear, cursor};
use termion::raw::{IntoRawMode};

fn draw_content(content_height: usize,
                comcfg:         &ComCfg,
                coucfg:         &CouCfg,
                content_lines:  &Vec<String>,
                scroll:         usize)
{	
	let mut line_index_begin: usize;
	let mut line_index_end: usize;

	if content_height < content_lines.len() {
		line_index_begin = scroll;
		line_index_end = scroll + content_height;
		
		if line_index_end >= content_lines.len() {
			line_index_begin -= line_index_end - content_lines.len();
			line_index_end -= line_index_end - content_lines.len();
		}
	} else {
		line_index_begin = 0;
		line_index_end = content_lines.len();
	} 
	
	for i in line_index_begin..line_index_end {
		print!("{}{}{}{}{}\n",
		       coucfg.colors.content.fg,
		       coucfg.colors.content.bg,
		       content_lines[i],
		       comcfg.colors.std.fg,
		       comcfg.colors.std.bg);
	}
}

#[must_use]
fn get_content(filepath: Option<String>) -> String {
	let mut f:    fs::File;
	let mut ret = String::new();

	let fp = match filepath {
	Some(tmp) => {
		tmp
	},
	
	None => {
		// TODO
		panic!("TODO implement piping");
	}};

	let result = fs::File::open(&fp);
	f = match result {
	Ok(f) => {
		f
	}
	
	Err(_) => {
		panic!("Could not open file \"{}\"", fp);
	}};
	
	let result = f.read_to_string(&mut ret);
	match result {
	Ok(_) => {}
	
	Err(_) => {
		panic!("Could not read file \"{}\"", fp);
	}}
	
	return ret;
}

#[must_use]
fn handle_cmd(active:            &mut bool,
              cmdline:           &mut String,
              content_lines_len: usize,
              scroll:            &mut usize)
              -> Option<String> // feedback is returned
{
	let mut ret: Option<String> = None;
	
	match cmdline as &str {
	"q" | "quit" | "exit" => {
		*active = false;
	}

	_ => {
		match usize::from_str_radix(cmdline.as_ref(), 10) {
		Ok(num) => {
			if num > 0 {
				if num > content_lines_len {
					*scroll = content_lines_len - 1;
				} else {
					*scroll = num - 1;
				}
			}
		}

		Err(_) => {
			ret = Some(format!("Command \"{}\" not recognised",
			                   cmdline));
		}}
	}}
	
	cmdline.clear();
	return ret;
}

fn handle_key(key:               char,
              active:            &mut bool,
              comcfg:            &ComCfg,
              content_lines_len: usize,
              cmdline:           &mut String,
              cmdmode:           &mut bool,
              feedback:          &mut Option<String>,
              scroll_limit:      usize,
              scroll:            &mut usize)
{
	if *cmdmode {
		if key == common::SIGINT || key == common::SIGTSTP {
			*cmdmode = false;
		} else if key == comcfg.keys.cmdenter {
			*feedback = handle_cmd(active,
			                       cmdline,
			                       content_lines_len,
			                       scroll);
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

#[must_use]
fn parse_args() -> String /* content */ {	
	let mut args: env::Args;
	let mut filepath: Option<String> = None;

	args = env::args();
	
	if args.len() > 1 {
		filepath = match args.nth(1) {
		Some(tmp) => Some(tmp),
		None => {
			panic!("Filepath argument doesn't exist");
		}};
	}
	
	return get_content(filepath);
}

fn main()
{
	let comcfg = ComCfg::from_file();
	let content: String;
	let coucfg = CouCfg::from_file();
	let title: String;

	let mut active = true;
	let mut cmdline: String = String::new();
	let mut cmdmode: bool = false;
	let mut content_lines = Vec::<String>::new();
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

	content = parse_args();
	stdin = std::io::stdin();
	let mut stdout = std::io::stdout().into_raw_mode().unwrap();
	// TODO temp val
	title = "Your ad could be here, for now!".to_string();

	while active {
		term_w_old = term_w;
		(term_w, term_h) = termion::terminal_size().unwrap();
		if term_w_old != term_w {
			content_lines = split_by_lines(&content, term_w);
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
		             &content_lines,
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
		           content_lines.len(),
		           &mut cmdline,
		           &mut cmdmode,
		           &mut feedback,
		           content_lines.len() - content_height,
		           &mut scroll);
	}
}
