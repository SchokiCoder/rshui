# v1.0.0

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

- child process execution test vim (maybe it already works)
- add courier
- give hui multiline feedback to courier

# v0.2.0

- test ron as config file format, if not toml
- read priority "/etc" over "~/.config/$FILE" over "~/.$FILE" over "$CWD/$FILE"
- if no config found, panic
- read config file
- remove source code config

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
- add config values for text fore- and background
- add config values for key binds

- the license into binary at compile time thing can be easily done with include
  macro?
- use shell scripts for building and deploying
- set version to 0.1
