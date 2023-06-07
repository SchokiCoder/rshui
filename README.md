# Rusted House User Interface

## What is this

A customizable terminal user-interface for common tasks and personal tastes,
inspired by suckless software.  
You can statically create TUI menus in config source code and then deploy it to
your user.  
Set it as their default shell, to chain them into specific tasks :D  
A scripting interface allows you to tack logic onto the menus.  
With it you can even create entire menus at runtime.  

## Version numbers

This is a rewrite of my [hui](https://github.com/SchokiCoder/hui) in Rust.  
It follows semantic versioning like the original but the minor and major number
depend on the features implemented.  
If rshui has the same features as hui 1.2.X then rshui is v1.2.X.  
Bugfixes are likely independent of each other, therefore the patchnumber is not
shared.  

# A suckless Rust app?

Yes, inspired by suckless and my original implementation.  
The Rust compiler and language design is very likely not approved by suckless
standards, but i want to see if it could be.  
  
My goal here is not to have hui in Rust but to write a Rust app with only a
subset of Rust's features.  
Maybe this can spark the question of: "Can one make a language like Rust but
simpler?"  
