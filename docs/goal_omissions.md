# Limitations

## User friendliness

- "configuration via Rust source code"
- "no dependcies on libraries other than std and core"
- "provide secondary goal updates as patches when not that necessary"

These limitations have been dropped to increase user friendliness.  
Simplicity makes C programs more maintainable and stable.  
Since i am using Rust, i figured that i can trade that for user friendliness and
still have plenty of stability to spare.  
Though I want the source code limitation to still be 3000 max for the sake of
development.  

## suckless Rust coding style

- no file level `use` except for local files and std::string::String
- no use of `impl` (thus no oop, traits or operator overloading)
- no closures
- no `unsafe`, except to create safe wrappers around C ffi

These limitations have been immature. I first want to get a feel for how they
could be implemented in suckless coding style before deciding on them.  
The document
[docs/code_style.md](https://github.com/SchokiCoder/rshui/blob/main/docs/code_style.md)
will now track my opinions about various Rust coding techniques in the context
of suckless coding.  
As i go, this document will add descriptions of what i discourage and what not.  

## no building via cargo

...i am lazy...  
It's an oversimplification.  
Rust brings more stability than C so it's fine.  
Bringing libraries into here would be painful otherwise.  
