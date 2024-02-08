# v1.1.0

- add specific feedback color for errors
- add feedback for when hitting "right" on a shell entry or "execute" on a menu
  entry
  "Entry type is \"menu\", cannot execute."
  "Entry type is \"shell\", cannot enter."
- handle return key in cmdline
- add cursor for each menu in menu_path
  Thus a "left" key press would send you to the menu entry that you entered.

# v1.0.0

- the license into binary at compile time thing can be easily done with include
  macro?
  Do i even still need that with GPL2?

- hui: fix entries being longer than a line causing overflows in available_space 

- mangen?
- config file manpage
- hui manpage
- courier manpage
- POSIX call options
- return values
- print messages (consistency, version information, license information)
- generalist standard configuration which says so itself via main menu title
- install scripts
- update README.md#Install

# v0.3.0

+ add support for child process execution with mainloop

+ split cfg code from hui

  Also add quit key to config (oversight).

+ add split old cfg into common cfg and hui cfg
+ finalize cfg split

+ just suspend raw mode when entering cmdline to get better input for free?
  No, because SIG handling is _not_ possible, i think.

+ courier: add basic drawing
+ courier: add raw mode
+ courier: add draw_lower
+ courier: add cmdline

+ courier: add content needed_lines awareness
  
This is for limiting the lines that can be printed at once.

+ hui: add needed_lines awareness for entries

+ courier: add scroll

Also minor code readability improvement for both hui and courier.

+ update matches to comply with code_style

+ courier: add scroll via entered cmdline number
+ courier: add read file argument
+ courier: add optional title argument
  the next argument after "-t" is a None?
  EVERYTHING after the 1st arg is a None?!
  Causes `courier -t title file` to not recognise filepath nor title.
  I have swapped args.nth(i) with arg in args and it just works.
  Conclusion: the rust std library can be cursed sometimes.

+ courier: fix using the first arg (path to executable) in parse_args
+ FAILED: courier: add content piping

I cannot reopen stdin from "dev/tty" like in the C version.
io::Stdin doesn't implement FromRawFd and the struct values are private.
I am literally gatekept from just tossing a filedesc at the stdin struct and
calling it a day.
Rust Std Library L.
I can not actually blame them.
This was a dumb hack to begin with.

- hui: fix Unrecognised command not clearing cmdline
- feedback needed_lines detection probably doesn't account for the cmdline
  prefix

- replace smug-ass over-abstracted termion::HideCursor nonsense with a simple
  function, so i can get my work done
  
  Doesn't cause detect_cursor issues but still ugly and over engineered.

- give hui multiline feedback to courier

- Maybe the ShellSession execution part can be modified to feasibly cover normal
  Shell execution, thus making it possible to remove the ShellSession vs Shell
  thing from the config.
  Using the Shell enum but giving something like vi, will freeze the terminal
  until SIGTERM.

# v0.2.0

+ write config.rs as toml
+ add config path priority
  "/etc" over "~/.config/$FILE" over "~/.$FILE" over "$CWD/$FILE"
  if no config found, panic

+ use new config structs

  Menus are now saved in the config struct via a Hashmap rather than Vec and name
  field for Menu.
  This also removes string references (`&str`) with actual `String`'s.
  This **may** be suboptimal but it works.
  
+ add config file read and adjust toml to new Hashmap

  Also remove source code config.

+ replace most unwraps

  Including a rewrite for cmdoutput.

+ update code style regarding match case brackets and apply to code

+ fix feedback not being emptied by a cmdline command with no output

+ fix menu navigation not resetting the cursor

+ add better toml format example
+ set version to 0.2.0
  
  Also update copyright.

# v0.1.0

+ README.md: remove ambigous patch statement
+ goals.md: add exception to file-level `use` limitation for local files

+ README.md: remove wrong statement
  "originally inspired by suckless software" -> "inspired by suckless software"
+ README.md: add rust clarification

+ add header
+ add title
+ add menu draw

+ remove cargo

+ add keyboard input and close via ctl + 'c' and 'q'
+ add cursor hide/show
+ add terminal clear
+ add raw terminal mode
  terminal freezes, because read_to_end waits for EOF... i am sleep deprived

+ carriage returns are currently necessary...
  termion::IntoRaw->raw_terminal_attr->libc::cfmakeraw
				       ^^^^^^^^^^^^^^^
				       sets attributes

  Even on musl, this sets way more flags than needed, which i believe to be the
  reason.
  Initially i wanted to replace termion::IntoRaw with libc::tcgetattr and
  libc::tcsetattr.
	+but suspending raw mode during draw works too :)

+ add menu entry cursor (up and down)
+ add basic shell command execution
+ add feedback line
+ add feedback color
+ fix incorrect default exec bind
+ move some main function parts into functions
+ detect needed lines for feedback and use that in draw_lower
+ add rtrim before needed_lines check
+ add menu navigation (left, right)
+ add entry prefix and postfix for each entry type
+ add command line enter via ':'
+ add command line leave via ctl + 'c'
+ add command line typing and display
+ add command interpretation via enter
+ add config sys for text fore- and background
+ add config values for key binds
+ set version to 0.1.0
