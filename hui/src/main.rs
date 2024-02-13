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
use termion::{clear};
use termion::raw::{IntoRawMode, RawTerminal};

fn cmdoutput_to_feedback(cmdoutput: Result<std::process::Output, std::io::Error>)
                         -> String // feedback
{
	return match cmdoutput {
	Ok(output) => {
		if output.stderr.len() > 0 {
			String::from_utf8(output.stderr).unwrap()
		} else {
			String::from_utf8(output.stdout).unwrap()
		}
	}

	Err(e) => {
		format!("Command output could not be retrieved: {}", e)
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
fn handle_cmd(active: &mut bool,
              cmdline: &String,
              cursor: &mut usize,
              huicfg: &HuiCfg,
              menu_path: &Vec<String>)
              -> String // feedback is returned
{
	let cur_menu: &Menu = &huicfg.menus[&menu_path[menu_path.len() - 1]];

	let mut ret = String::new();

	match cmdline as &str {
	"q" | "quit" | "exit" => {
		*active = false;
	}

	_ => {
		match usize::from_str_radix(cmdline.as_ref(), 10) {
		Ok(num) => {
			if num >= cur_menu.entries.len() {
				*cursor = cur_menu.entries.len() - 1;
			} else {
				*cursor = num;
			}
		}

		Err(_) => {
			ret = format!("Command \"{}\" not recognised", cmdline);
		}}
	}}

	return ret;
}

fn handle_entry_execution(entry_content: &EntryContent) -> String // feedback
{
	let mut ret = String::new();

	match entry_content {
	EntryContent::Shell(cmdstr) => {
		let cresult = Command::new("sh")
		                      .arg("-c")
		                      .arg(cmdstr)
		                      .output();

		ret = cmdoutput_to_feedback(cresult);
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
				ret = "Couldn't run child process.".to_string();
			}}
		}

		Err(_) => {
			ret = "Couldn't spawn child process.".to_string();
		}}
	}

	_ => {}
	}

	return ret;
}

fn handle_key(key:       char,
              active:    &mut bool,
              cmdline:   &mut String,
              cmdmode:   &mut bool,
              comcfg:    &ComCfg,
              cursor:    &mut usize,
              feedback:  &mut String,
              huicfg:    &HuiCfg,
              menu_path: &mut Vec<String>)
{
	let cur_menu = &huicfg.menus[&menu_path[menu_path.len() - 1]];

	if *cmdmode {
		*cmdline = handle_key_cmdmode(key,
		                              active,
		                              cmdline,
		                              cmdmode,
		                              comcfg,
		                              cursor,
		                              feedback,
		                              huicfg,
		                              menu_path);
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
		*feedback = handle_entry_execution(&cur_menu.entries[*cursor].content);
	} else if key == comcfg.keys.cmdmode {
		if *cmdmode == false {
			*cmdmode = true;
		}
	}
}

#[must_use]
fn handle_key_cmdmode(key:               char,
                      active:            &mut bool,
                      cmdline:           &String,
                      cmdmode:           &mut bool,
                      comcfg:            &ComCfg,
                      cursor:            &mut usize,
                      feedback:          &mut String,
                      huicfg:            &HuiCfg,
                      menu_path:         &Vec<String>)
                      -> String // cmdline
{
	let ret: String;

	if key == common::SIGINT || key == common::SIGTSTP {
		*cmdmode = false;
		ret = String::new();
	} else if key == comcfg.keys.cmdenter {
		*feedback = handle_cmd(active,
		                       cmdline,
		                       cursor,
		                       huicfg,
		                       menu_path);
		ret = String::new();
		*cmdmode = false;
	} else {
		ret = format!("{}{}", *cmdline, key)
	}
	
	return ret;
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
	let mut feedback = String::new();
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
			header_lines = split_by_lines(&huicfg.header, term_w);
			title_lines = split_by_lines(&cur_menu.title, term_w);
		}
		content_height = term_h as usize -
		                 header_lines.len() -
		                 title_lines.len() -
		                 1 - 1;

		print!("{}", clear::All);
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
			   &mut feedback,
			   term_w,
			   term_h);
		
		stdout.flush().expect("Stdout flush failed");
		stdout.activate_raw_mode().unwrap();

		stdin.read_exact(&mut input).expect("Keyboard read failed");
		handle_key(input[0] as char,
		           &mut active,
		           &mut cmdline,
		           &mut cmdmode,
		           &comcfg,
		           &mut cursor,
		           &mut feedback,
		           &huicfg,
		           &mut menu_path);
	}
}
