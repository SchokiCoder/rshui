// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023 - 2024  Andy Frank Schoknecht

mod config;
mod menu;

use crate::config::HuiCfg;
use crate::menu::*;

use common::*;
use common::config::ComCfg;
use std::io::{Read, Write};
use std::process::Command;
use termion::{clear, cursor};
use termion::raw::{IntoRawMode, RawTerminal};

fn cmdoutput_to_feedback(cmdoutput: Result<std::process::Output, std::io::Error>)
                         -> Option<String>
{
	return match cmdoutput {
	Ok(output) => {
		if output.stderr.len() > 0 {
			Some(String::from_utf8(output.stderr).unwrap())
		} else {
			Some(String::from_utf8(output.stdout).unwrap())
		}
	}

	Err(e) => {
		Some(format!("Command output could not be retrieved: {}", e))
	}}
}

fn draw_menu(content_height: usize,
             menu: &Menu,
             comcfg: &ComCfg,
             huicfg: &HuiCfg,
             cursor: usize)
{
	let mut caption: &String;
	let mut entry_index_begin: usize;
	let mut entry_index_end: usize;
	let mut prefix:  &String;
	let mut postfix: &String;

	if content_height < menu.entries.len() {
		entry_index_begin = cursor;
		entry_index_end = cursor + content_height;
		
		if entry_index_end >= menu.entries.len() {
			entry_index_begin -= entry_index_end - menu.entries.len();
			entry_index_end -= entry_index_end - menu.entries.len();
		}
	} else {
		entry_index_begin = 0;
		entry_index_end = menu.entries.len();
	}
	
	for i in entry_index_begin..entry_index_end {
		caption = &menu.entries[i].caption;
		
		match menu.entries[i].content {
		EntryContent::Menu(_) => {
			prefix  = &huicfg.entry_menu_prefix;
			postfix = &huicfg.entry_menu_postfix;
		}

		EntryContent::Shell(_) => {
			prefix  = &huicfg.entry_shell_prefix;
			postfix = &huicfg.entry_shell_postfix;
		}

		EntryContent::ShellSession(_) => {
			prefix  = &huicfg.entry_shellsession_prefix;
			postfix = &huicfg.entry_shellsession_postfix;
		}}
		
		if i == cursor {
			print!("{}{}{}{}{}{}{}\n",
			       huicfg.colors.entry_hover.fg,
			       huicfg.colors.entry_hover.bg,
			       prefix,
			       caption,
			       postfix,
			       comcfg.colors.std.fg,
			       comcfg.colors.std.bg);
		} else {
			print!("{}{}{}{}{}{}{}\n",
			       huicfg.colors.entry.fg,
			       huicfg.colors.entry.bg,
			       prefix,
			       caption,
			       postfix,
			       comcfg.colors.std.fg,
			       comcfg.colors.std.bg);
		}
	}
}

#[must_use]
fn handle_cmd(cmdline: &mut String,
              active: &mut bool,
              cfg: &HuiCfg,
              cursor: &mut usize,
              menu_path: &mut Vec<String>)
              -> Option<String> // feedback is returned
{
	let cur_menu: &Menu = &cfg.menus[&menu_path[menu_path.len() - 1]];

	let mut ret: Option<String> = None;

	match cmdline as &str {
	"q" | "quit" | "exit" => {
		*active = false;
	}

	_ => {
		match usize::from_str_radix(cmdline.as_ref(), 10) {
		Ok(num) => {
			if num > 0 {
				if num > cur_menu.entries.len() {
					*cursor = cur_menu.entries.len() - 1;
				} else {
					*cursor = num - 1;
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

fn handle_key(key:       char,
              active:    &mut bool,
              comcfg:    &ComCfg,
              huicfg:    &HuiCfg,
              cmdline:   &mut String,
              cmdmode:   &mut bool,
              cursor:    &mut usize,
              feedback:  &mut Option<String>,
              menu_path: &mut Vec<String>)
{
	let cur_menu = &huicfg.menus[&menu_path[menu_path.len() - 1]];

	if *cmdmode {
		if key == common::SIGINT || key == common::SIGTSTP {
			*cmdmode = false;
		} else if key == comcfg.keys.cmdenter {
			*feedback = handle_cmd(cmdline,
			                       active,
			                       huicfg,
			                       cursor,
			                       menu_path);
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
	} else if key == comcfg.keys.down {
		if *cursor < (cur_menu.entries.len() - 1) {
			*cursor += 1;
		}
	} else if key == comcfg.keys.up {
		if *cursor > 0 {
			*cursor -= 1;
		}
	} else if key == comcfg.keys.right {
		match &cur_menu.entries[*cursor].content {
		EntryContent::Menu(m) => {
			menu_path.push(m.to_string());
			*cursor = 0;
		}
		
		_ => {}
		}
	} else if key == comcfg.keys.left {
		if menu_path.len() > 1 {
			menu_path.pop();
			*cursor = 0;
		}
	} else if key == huicfg.keys.execute {
		match &cur_menu.entries[*cursor].content {
		EntryContent::Shell(cmdstr) => {
			let cresult = Command::new("sh")
			                      .arg("-c")
			                      .arg(cmdstr)
			                      .output();

			*feedback = cmdoutput_to_feedback(cresult);
		}

		EntryContent::ShellSession(cmdstr) => {
			let cmdspawn = Command::new("sh")
			                       .stdout(std::io::stdout())
			                       .arg("-c")
			                       .arg(cmdstr)
			                       .spawn();
			match cmdspawn {
			Ok(child) => {
				match child.wait_with_output() {
				Ok(_) => {}
				Err(_) => {
					Some("Couldn't run child process.".to_string());
				}}
			}

			Err(_) => {
				*feedback = Some("Couldn't spawn child process.".to_string());
			}}
		}

		_ => {}
		}
	} else if key == comcfg.keys.cmdmode {
		if *cmdmode == false {
			*cmdmode = true;
		}
	}
}

fn main()
{
	let comcfg = ComCfg::from_file();
	let huicfg = HuiCfg::from_file();

	let mut active: bool = true;
	let mut cursor: usize = 0;
	let mut cmdline: String = String::new();
	let mut cmdmode: bool = false;
	let mut content_height: usize;
	let mut feedback: Option<String> = None;
	let mut header_lines = Vec::<String>::new();
	let mut input: [u8; 1] = [0];
	let mut stdin: std::io::Stdin;
	let mut stdout: RawTerminal<std::io::Stdout>;
	let mut term_w: u16 = 0;
	let mut term_w_old: u16;
	let mut term_h: u16;
	let mut title_lines = Vec::<String>::new();
	let mut menu_path: Vec<String> = vec!["main".to_string()];

	stdout = std::io::stdout().into_raw_mode().unwrap();
	stdin = std::io::stdin();
	
	while active {
		let cur_menu = &huicfg.menus[&menu_path[menu_path.len() - 1]];

		term_w_old = term_w;
		(term_w, term_h) = termion::terminal_size().unwrap();
		if term_w_old != term_w {
			// TODO temp val
			header_lines = split_by_lines(&huicfg.header, term_w);
			title_lines = split_by_lines(&cur_menu.title, term_w);
		}
		content_height = term_h as usize -
		                 header_lines.len() -
		                 title_lines.len() -
		                 1 - 1;

		print!("{}", clear::All);
		print!("{}", cursor::Goto(1, 1));
		stdout.suspend_raw_mode().unwrap();

		draw_upper(&comcfg, &header_lines, &title_lines);
		draw_menu(content_height,
		          &cur_menu,
		          &comcfg,
		          &huicfg,
		          cursor);
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
		           &huicfg,
		           &mut cmdline,
		           &mut cmdmode,
		           &mut cursor,
		           &mut feedback,
		           &mut menu_path);
	}
}
