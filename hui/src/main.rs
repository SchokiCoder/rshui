// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2023 - 2024  Andy Frank Schoknecht

mod config;
mod menu;

use crate::config::HuiCfg;
use crate::menu::*;

use common::config::ComCfg;
use std::io::{Read, Write};
use std::process::Command;
use termion::{clear, cursor};
use termion::cursor::{DetectCursorPos, HideCursor};
use termion::raw::IntoRawMode;

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
		}
	};
}

fn draw_feedback(feedback: &Option<String>, cfg: &HuiCfg, term_w: u16)
{
	let fb_str = match feedback {
	Some(x) => {
		x
		}
	
	None => {
		return;
		}
	};

	let fb_str = fb_str.trim_end();
	if get_needed_lines(fb_str, term_w as usize) != 1 {
		return;
	}

	print!("{}{}{}{}{}",
	       cfg.colors.feedback.fg,
	       cfg.colors.feedback.bg,
	       fb_str,
	       cfg.colors.std.fg,
	       cfg.colors.std.bg);
}

fn draw_lower(cfg: &HuiCfg,
	      cmdline: &String,
              cmdmode: &bool,
              feedback: &Option<String>,
              stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
              term_w: u16,
              term_h: u16)
{
	let y: u16;

	stdout.activate_raw_mode().unwrap();
	(_, y) = stdout.cursor_pos().unwrap();
	stdout.suspend_raw_mode().unwrap();

	for _ in y..term_h {
		print!("\n");
	}
	
	print!("{}{}:{}{}",
	       cfg.colors.feedback.fg,
	       cfg.colors.feedback.bg,
	       cfg.colors.std.fg,
	       cfg.colors.std.bg);

	if *cmdmode {
		print!("{}{}{}{}{}",
		       cfg.colors.cmdline.fg,
		       cfg.colors.cmdline.bg,
		       cmdline,
		       cfg.colors.std.fg,
		       cfg.colors.std.bg);
	} else {
		draw_feedback(feedback, cfg, term_w);
	}
}

fn draw_menu(menu: &Menu, cfg: &HuiCfg, cursor: usize)
{
	let mut prefix:  &String;
	let mut caption: &String;
	let mut postfix: &String;

	for i in 0..menu.entries.len() {
		caption = &menu.entries[i].caption;
		
		match menu.entries[i].content {
		EntryContent::Menu(_) => {
			prefix  = &cfg.entry_menu_prefix;
			postfix = &cfg.entry_menu_postfix;
			}

		EntryContent::Shell(_) => {
			prefix  = &cfg.entry_shell_prefix;
			postfix = &cfg.entry_shell_postfix;
			}

		EntryContent::ShellSession(_) => {
			prefix  = &cfg.entry_shellsession_prefix;
			postfix = &cfg.entry_shellsession_postfix;
			}
		}
		
		if i == cursor {
			print!("{}{}{}{}{}{}{}\n",
			       cfg.colors.entry_hover.fg,
			       cfg.colors.entry_hover.bg,
			       prefix,
			       caption,
			       postfix,
			       cfg.colors.std.fg,
			       cfg.colors.std.bg);
		} else {
			print!("{}{}{}{}{}{}{}\n",
			       cfg.colors.entry.fg,
			       cfg.colors.entry.bg,
			       prefix,
			       caption,
			       postfix,
			       cfg.colors.std.fg,
			       cfg.colors.std.bg);
		}
	}
}

fn draw_upper(cfg: &HuiCfg, title: &String)
{
	print!("{}{}{}{}{}\n",
	       cfg.colors.header.fg,
	       cfg.colors.header.bg,
	       cfg.header,
	       cfg.colors.std.fg,
	       cfg.colors.std.bg);

	print!("{}{}{}{}{}\n",
	       cfg.colors.title.fg,
	       cfg.colors.title.bg,
	       title,
	       cfg.colors.std.fg,
	       cfg.colors.std.bg);
}

fn get_needed_lines(s: &str, line_width: usize) -> usize
{
	let mut ret: usize = 1;
	let mut x: usize = 0;

	if s.len() == 0 {
		return 0;
	}

	for c in s.bytes() {
		match c as char {
		'\r' | '\n' => {
			ret += 1;
			x = 0;
			}

		_ => { x += 1; }
		}

		if x > line_width {
			x = 0;
			ret += 1;
		}
	}

	ret
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
			},

		Err(_) => {
			return Some(format!("Command \"{}\" not recognised",
			                    cmdline));
			},
		}
		}
	}
	
	cmdline.clear();
	return None;
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
					}
				}
				}

			Err(_) => {
				*feedback = Some("Couldn't spawn child process.".to_string());
				}
			}
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
	let mut feedback: Option<String> = None;
	let mut input: [u8; 1] = [0];
	let mut stdin: std::io::Stdin;
	let mut stdout: termion::cursor::HideCursor<
		termion::raw::RawTerminal<std::io::Stdout>>;
	let mut term_w: u16;
	let mut term_h: u16;
	let mut menu_path: Vec<String> = vec!["main".to_string()];

	stdout = HideCursor::from(std::io::stdout().into_raw_mode().unwrap());
	stdin = std::io::stdin();
	
	while active {
		let cur_menu = &huicfg.menus[&menu_path[menu_path.len() - 1]];

		(term_w, term_h) = termion::terminal_size().unwrap();

		print!("{}", clear::All);
		print!("{}", cursor::Goto(1, 1));
		stdout.suspend_raw_mode().unwrap();

		draw_upper(&huicfg, &cur_menu.title);
		draw_menu(&cur_menu, &huicfg, cursor);

		draw_lower(&huicfg,
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
