```
j6      j6
QQ      4P
QQ
QQ
QQQQQ6  jg
QQQQQQ  QQ
QQ  QQ  QQ
QQ  QQ  QQ
QQ  QQQQQQ
4P  4QQQQP
```

# What is this?

A **cancelled** Rust rewrite of [hui](https://github.com/SchokiCoder/hui).  
A customizable terminal user-interface for common tasks and personal tastes,
inspired by suckless software.  
You can create TUI menus in a config file and then deploy it to your user.  
Then set hui as their default shell, to chain them into specific tasks :D  
A scripting interface allows you to tack logic onto the menus.  
With it you can even create entire menus at runtime.  

# HUI

This project is a Rust rewrite of
[the original hui](https://github.com/SchokiCoder/hui).  
The purpose of this to myself is to figure out how suckless coding style can be
applied to a Rust project.  
Other than that it intends to be a application useful for average to expert
Linux/BSD/etc users.  
So this will reimplement all of the original features plus a config file system.  

# Install (no scripts yet, won't work)

Follow these steps:  

- `git clone https://github.com/SchokiCoder/rshui`
- `cd rshui`
- `chmod u+x *.sh; sudo ./install.sh`

This will install two binaries "hui" and "courier".  
Courier is the pager that also lives here, because they share a lot of code so
they can look and feel similar.  
If you don't wish to have "courier", edit
[install.sh](https://github.com/SchokiCoder/rshui/blob/main/install.sh).  
