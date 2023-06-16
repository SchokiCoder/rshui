/* Copyright (C) 2023 Andy Frank Schoknecht
 * Use of this source code is governed by the BSD-3-Clause
 * license, that can be found in the LICENSE file.
 */

mod menu;
mod config;

use crate::menu::*;
use crate::config::*;

fn draw_menu(menu: &Menu)
{
	for entry in menu.entries {
		println!("{}{}", ENTRY_PREPEND, entry.caption);
	}
}

fn main()
{
	'mainloop: loop {
		println!("{}", HEADER);
		println!("{}", MENU_MAIN.title);
		
		draw_menu(&MENU_MAIN);
		
		break 'mainloop;
	}
}
