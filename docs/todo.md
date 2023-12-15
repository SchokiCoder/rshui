# v0.2

- add courier

# v0.1

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
- add feedback color
- detect needed lines for feedback and use that in draw_lower
- add menu navigation (left, right)
- add command line enter via ':'
- add command line leave via ctl + 'c' and enter
- add config for text fore- and background

- look at asm to find more things that need to be set for rustc
  `rustc --emit asm`

- the license into binary at compile time thing can be easily done with include
  macro?
- use shell scripts for building and deploying
- resolve limitation violations (Cargo, lib "termion")
- suckless code style in rust sucks (regarding matches)
- set version to 0.1
